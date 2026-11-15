const { chromium, webkit } = require(process.env.PLAYWRIGHT_MODULE || 'playwright');
const assert = require('node:assert/strict');
(async () => {
for (const engine of [chromium, webkit]) {
 const browser=await engine.launch({headless:true, executablePath: process.env[engine.name().toUpperCase() + '_EXECUTABLE']});
 const page=await browser.newPage();const errors=[];page.on('pageerror',e=>errors.push(String(e)));
 await page.goto(process.env.BASE_URL);await page.waitForFunction(()=>window.ready, null, {timeout: 10000});
 assert.equal(await page.evaluate(()=>{bridge.enable(false,false);return document.activeElement.tagName}),'CANVAS');
 await page.click('#open');assert.equal(await page.evaluate(()=>document.activeElement.dataset.icedIme),'');
 await page.keyboard.type('Rust');
 assert.deepEqual(await page.evaluate(()=>bridge.events()),['Commit("R")','Commit("u")','Commit("s")','Commit("t")']);
 assert.deepEqual(await page.evaluate(()=>keys),[]);
 await page.keyboard.press('Backspace');
 assert.deepEqual(await page.evaluate(()=>keys.filter(e=>e.type==='keydown').map(e=>e.key)),['Backspace']);
 assert.deepEqual(await page.evaluate(()=>bridge.events()),[]);
 await page.evaluate(()=>{keys=[];document.querySelector('[data-iced-ime]').dispatchEvent(new InputEvent('beforeinput',{inputType:'deleteContentBackward',cancelable:true}));});
 assert.deepEqual(await page.evaluate(()=>keys.filter(e=>e.type==='keydown').map(e=>e.key)),['Backspace']);
 await page.keyboard.press('Shift+ArrowLeft');assert((await page.evaluate(()=>keys)).some(e=>e.key==='ArrowLeft'&&e.shift));
 await page.evaluate(()=>{const input=document.querySelector('[data-iced-ime]');input.dispatchEvent(new CompositionEvent('compositionstart'));input.value='\u200bに';input.dispatchEvent(new CompositionEvent('compositionupdate',{data:'に'}));input.dispatchEvent(new InputEvent('input',{inputType:'insertCompositionText',data:'に',isComposing:true}));input.dispatchEvent(new CompositionEvent('compositionend',{data:'日本'}));input.value='\u200b日本';input.dispatchEvent(new InputEvent('input',{inputType:'insertFromComposition',data:'日本'}));});
 const composition=await page.evaluate(()=>bridge.events());assert.deepEqual(composition,['Opened','Preedit("に", Some(3..3))','Preedit("", None)','Commit("日本")','Closed']);
 await page.evaluate(()=>{const input=document.querySelector('[data-iced-ime]');input.value='\u200bclipboard ✨';input.dispatchEvent(new InputEvent('input',{inputType:'insertFromPaste'}));});
 assert.deepEqual(await page.evaluate(()=>bridge.events()),['Commit("clipboard ✨")']);
 assert.equal(await page.evaluate(()=>{bridge.enable(false,true);return document.querySelector('[data-iced-ime]').type}),'password');
 await page.evaluate(()=>bridge.disable());assert.equal(await page.evaluate(()=>document.activeElement.tagName),'CANVAS');
 await page.evaluate(()=>{focusEvents=[];bridge.enable(true,false);});assert.deepEqual(await page.evaluate(()=>focusEvents),[]);
 await page.evaluate(()=>bridge.dispose());assert.equal(await page.locator('[data-iced-ime]').count(),0);assert.deepEqual(errors,[]);
 console.log(engine.name()+': all DOM input/focus/composition/shortcut tests passed');await browser.close();
}
})().catch(e=>{console.error(e);process.exit(1)});
