#!/usr/bin/env python3
"""Compile every rust block in examples/browser/snippets/*.md.

Each snippet becomes a module of examples/browser/tests/snippets.rs
(gitignored), checked with `cargo check -p browser --test snippets`, so
it builds against the same iced and icedcn features as the browser. A
block's loose statements are wrapped in a function returning a View, and
a Message enum is generated from the variants it names. Names a snippet
leaves to its page come from scripts/snippet-stubs.py.

Exits non-zero when a snippet fails, listing each error by snippet. The
generated file is removed afterwards; pass --keep to read it.
"""
import bisect, collections, re, glob, json, os, subprocess, sys
ROOT=os.path.dirname(os.path.dirname(os.path.abspath(__file__)))
OUT=ROOT+'/examples/browser/tests/snippets.rs'
SNIP=ROOT+'/examples/browser/snippets'
FENCE=re.compile(r"(?m)^```rust[^\n]*\n(.*?)^```",re.S)
NS={'ROOT': ROOT}
exec(open(ROOT+'/scripts/snippet-stubs.py').read(),NS)
OV=NS['OV']; PAGES=NS['PAGES']

def depth_scan(line, state):
    """Return depth delta for a line, tracking strings/comments across lines. state: dict."""
    d=0; i=0; n=len(line)
    while i<n:
        c=line[i]
        if state.get('str'):
            q=state['str']
            if q=='"':
                if c=='\\': i+=2; continue
                if c=='"': state['str']=None
            else:  # raw: q is '"'+'#'*k
                if line.startswith(q,i): state['str']=None; i+=len(q); continue
            i+=1; continue
        if state.get('block'):
            if line.startswith('*/',i): state['block']=False; i+=2; continue
            i+=1; continue
        if line.startswith('//',i): break
        if line.startswith('/*',i): state['block']=True; i+=2; continue
        m=re.match(r'(?:b|c)?r(#*)"',line[i:])
        if m and (i==0 or not (line[i-1].isalnum() or line[i-1]=='_')):
            state['str']='"'+m.group(1); i+=len(m.group(0)); continue
        if c=='"': state['str']='"'; i+=1; continue
        if c=="'":
            m=re.match(r"'(\\.|\\u\{[0-9a-fA-F]+\}|[^\\'])'",line[i:])
            if m: i+=len(m.group(0)); continue
            i+=1; continue
        if c in '([{': d+=1
        elif c in ')]}': d-=1
        i+=1
    return d

ITEM=re.compile(r'^(#\[|#!\[|(pub(\([a-z]+\))?\s+)?(use|fn|struct|enum|impl|const|static|type|mod|trait|macro_rules!|async fn|unsafe)\b|[A-Za-z_][\w:]*!\s*\{)')

def split(block):
    lines=block.splitlines()
    chunks=[]; cur=[]; depth=0; st={}
    for l in lines:
        if depth==0 and not st.get('str') and not st.get('block') and l and not l[0].isspace() and l[0] not in ')]}.?' and cur:
            # new chunk unless the previous chunk is an attribute line or continuation
            prev=cur[-1].strip()
            if not (prev.startswith('#[') and prev.endswith(']')) and not prev.startswith('//') and not l.startswith(('where','{')) and not prev.endswith(('=', '+', '&&', '||', '(', ',')):
                chunks.append(cur); cur=[]
        cur.append(l); depth+=depth_scan(l,st)
    if cur: chunks.append(cur)
    items=[]; stmts=[]
    for ch in chunks:
        first=next((x for x in ch if x.strip() and not x.strip().startswith('//')),'')
        (items if ITEM.match(first) else stmts).append('\n'.join(ch))
    return items, stmts

def split_args(inner):
    args=[]; d=0; cur=''; st=None
    for c in inner:
        if st:
            cur+=c
            if c=='"' and not cur.endswith('\\"'): st=None
            continue
        if c=='"': st='"'; cur+=c; continue
        if c in '([{': d+=1
        elif c in ')]}': d-=1
        if c==',' and d==0: args.append(cur.strip()); cur=''
        else: cur+=c
    if cur.strip(): args.append(cur.strip())
    return args

def argtype(a):
    if re.fullmatch(r'"(?:[^"\\]|\\.)*"',a): return "&'static str"
    if re.fullmatch(r'-?\d+',a): return 'usize'
    if re.fullmatch(r'-?\d+\.\d*(f32)?',a): return 'f32'
    if a in ('true','false'): return 'bool'
    if re.fullmatch(r'"[^"]*"\.(into|to_string|to_owned)\(\)',a) or a.startswith('String::') or a.startswith('format!'): return 'String'
    return None

def variants(text, defined, en='Message'):
    out={}
    for m in re.finditer(r'\b'+en+r'::([A-Z]\w*)(\s*\(|\s*\{)?',text):
        name=m.group(1)
        if m.group(2) and '(' in m.group(2):
            j=m.end(); d=1
            while j<len(text) and d:
                c=text[j]
                if c in '([{': d+=1
                elif c in ')]}': d-=1
                j+=1
            args=split_args(text[m.end():j-1])
            if not args: continue
            prev=out.get(name)
            tys=[argtype(a) for a in args]
            if isinstance(prev,list) and len(prev)==len(tys):
                tys=[t or p for t,p in zip(tys,prev)]
            out[name]=tys
        elif m.group(2):
            out.setdefault(name,'struct')
        else:
            out.setdefault(name,None)
    return out

def render(en,vs):
    body=[]
    for v,a in vs.items():
        if a is None or a==0 or a=='unit': body.append(f'{v},')
        elif a=='struct': body.append(f'{v} {{}},')
        elif isinstance(a,str): body.append(f'{v}({a}),')
        else: body.append(f'{v}('+', '.join(t or 'String' for t in a)+'),')
    return '#[derive(Debug, Clone)] pub enum '+en+' { '+' '.join(body)+' }'

ITEMNAME=re.compile(r'(?m)^(?:pub(?:\([a-z]+\))?\s+)?(?:const\s+fn|async\s+fn|fn|struct|enum|const|static|type|trait|mod)\s+(\w+)')
PROMOTE=re.compile(r'(?m)^((?:const\s+fn|async\s+fn|fn|struct|enum|const|static|type|trait|mod)\s)')

def pages():
    m={}
    for f in glob.glob(ROOT+'/examples/browser/src/**/*.rs',recursive=True):
        for n in re.findall(r'examples/browser/snippets/([a-z0-9-]+)\.md',open(f).read()):
            m.setdefault(n,os.path.basename(f))
    return m

def main():
    pm=pages()
    blocks=[]
    for f in sorted(glob.glob(SNIP+'/*.md')):
        name=os.path.basename(f)[:-3]
        for i,b in enumerate(FENCE.findall(open(f).read())):
            key=f'{name}#{i}'
            b=OV.get(key,{}).get('replace_block',b)
            # Snippets name assets from src/component/; the test sits in tests/.
            b=b.replace('"../../assets/','"../assets/')
            blocks.append((name,i,b))
    out=['#![allow(unused, dead_code, unused_imports, non_snake_case, non_camel_case_types, private_interfaces, clippy::all)]',
         '#[path = "../src/icon.rs"] mod icon;']
    fdef={}
    for (name,i,b) in blocks:
        fdef.setdefault(name,{'defined':False,'vars':{}})
        if re.search(r'\b(enum|struct|type)\s+Message\b',b): fdef[name]['defined']=True
        fdef[name]['vars'].update(variants(b,False))
    index={}  # key -> (page, path, itemnames)
    path=[]; prev=None
    for idx,(name,i,b) in enumerate(blocks):
        if name!=prev: path=[]; prev=name
        path=path+[f'snippet_{idx}']
        items,_=split(b)
        names=set()
        for it in items:
            if not it.lstrip().startswith(('use','pub use','impl')): names|=set(ITEMNAME.findall(it))
        index[f'{name}#{i}']=(pm.get(name,name),'crate::'+'::'.join(path),names)
    open_depth=0; prev_name=None
    for idx,(name,i,b) in enumerate(blocks):
        if name!=prev_name:
            out.append('}'*open_depth); open_depth=0; prev_name=name
        key=f'{name}#{i}'
        ov=OV.get(key,{})
        if ov.get('skip'): out.append(f'// {key} skipped'); continue
        items,stmts=split(b)
        items=[PROMOTE.sub(r'pub(crate) \1',it,count=1) if not it.lstrip().startswith(('use','pub','impl','#![')) and not re.match(r'^[A-Za-z_][\w:]*!',it) else it for it in items]
        defined=bool(re.search(r'\b(enum|struct|type)\s+Message\b',b)) or bool(re.search(r'(?m)^use [^;]*\bMessage\b',b))
        mod=[f'// {key}', f'mod snippet_{idx} {{', '#![allow(unused)]']
        if i>0: mod.append('use super::*;')
        pg=pm.get(name,name)
        if pg in PAGES and i==0: mod.append(f'use crate::__page_{pg[:-3]}::*;')
        mod+=[re.sub(r'\{path:([^}]+)\}',lambda m: index[m.group(1)][1],x) for x in ov.get('prelude',[])]
        if not ov.get('no_message') and ((i==0 and not fdef[name]['defined'] and not defined) or ov.get('gen_message')):
            vs=dict(fdef[name]['vars'])
            vs.update(ov.get('variants',{}))
            mod.append(render('Message',vs))

        for en in ('Event',):
            if re.search(r'\b'+en+'::',b) and not re.search(r'\b(enum|struct|type)\s+'+en+r'\b',b) and not ov.get('no_'+en):
                vs=variants(b,False,en); vs.update(ov.get(en,{}))
                mod.append(render(en,vs))
        mod+=items
        if stmts:
            ret=ov.get('ret')
            if ret is None and re.search(r'\bEvent::',b) and not re.search(r'\bMessage::',b): ret_msg='Event'
            else: ret_msg='Message'
            ret_msg=ov.get('msg',ret_msg)
            body='\n'.join(ov.get('locals',[])+stmts)
            selfp='&self' if re.search(r'\bself\b',body) else ''
            if ret=='()' or (ret is None and body.rstrip().endswith(';')):
                fn=f'fn __body({selfp}) {{\n{body}\n}}'
            elif ret and ov.get('into'):
                fn=f'fn __body({selfp}) -> {ret} {{\nlet __v = {{\n{body}\n}};\n__v.into()\n}}'
            elif ret:
                fn=f'fn __body({selfp}) -> {ret} {{\n{body}\n}}'
            elif body.rstrip().endswith('.into()'):
                lt="'_" if selfp else "'static"
                fn=f"fn __body({selfp}) -> icedcn::View<{lt}, {ret_msg}> {{\n{body}\n}}"
            else:
                lt="'_" if selfp else "'static"
                fn=f"fn __body({selfp}) -> icedcn::View<{lt}, {ret_msg}> {{\nlet __v = {{\n{body}\n}};\n__v.into()\n}}"
            if selfp:
                fields=ov.get('fields',{})
                fn='pub struct __State { '+', '.join(f'pub {k}: {v}' for k,v in fields.items())+' }\nimpl __State {\n'+fn+'\n'+ov.get('state_impl','')+'\n}'
            mod.append(fn)
        open_depth+=1
        out+=mod
    out.append('}'*open_depth)
    for pg,code in PAGES.items():
        out.append(f'mod __page_{pg[:-3]} {{\n#![allow(unused)]\nuse iced::widget::{{Text, image}};\nuse icedcn::{{Theme, View}};\nuse crate::icon;\n{code}\n}}')
    out+=[re.sub(r'\{path:([^}]+)\}',lambda m: index[m.group(1)][1],x) for x in NS.get('ROOT_EXTRA',[])]
    open(OUT,'w').write('\n'.join(out)+'\n')
    return len(blocks)

def report(source, messages):
    starts=[(n,m.group(1)) for n,l in enumerate(source.splitlines(),1) if (m:=re.match(r'// (\S+#\d+)$',l))]
    lines=[s[0] for s in starts]
    errs=collections.defaultdict(list)
    for m in messages:
        spans=[s for s in m['spans'] if s['is_primary'] and s['file_name'].endswith('snippets.rs')]
        if not spans: errs['(outside a snippet)'].append(m['rendered'] or m['message']); continue
        s=spans[0]; i=bisect.bisect_right(lines,s['line_start'])-1
        key=starts[i][1] if i>=0 else '(harness)'
        text=s['text'][0]['text'].strip() if s['text'] else ''
        errs[key].append(f"{m['message']}: {text}")
    for k,v in sorted(errs.items()):
        print(f'{k}:')
        for e in v: print('   ',e)
    return sum(len(v) for v in errs.values())

if __name__=='__main__':
    n=main()
    run=subprocess.run(['cargo','check','-p','browser','--test','snippets','--message-format=json'],cwd=ROOT,capture_output=True,text=True)
    messages=[]
    for l in run.stdout.splitlines():
        try: j=json.loads(l)
        except ValueError: continue
        if j.get('reason')=='compiler-message' and j['message']['level']=='error': messages.append(j['message'])
    errors=report(open(OUT).read(), messages)
    # A file in tests/ is a test target: left behind, it would fail `cargo fmt --check`.
    if '--keep' not in sys.argv: os.remove(OUT)
    if run.returncode and not errors: sys.stderr.write(run.stderr); sys.exit(run.returncode)
    print(f'{n} blocks, {errors} errors')
    sys.exit(1 if errors else 0)
