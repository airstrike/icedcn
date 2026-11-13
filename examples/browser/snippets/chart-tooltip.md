```rust
fn sample(title:Option<&'static str>, rows:&[(&'static str,&'static str,Color)],style:&str)->Slot<'static,Message>{
 let mut items=Vec::new();if let Some(title)=title{items.push(text::small(title).medium().into());}
 for &(name,value,color) in rows{
  let swatch=|width,height|container(space::horizontal()).width(width).height(height).style(move |_|Style{background:Some(color.into()),border:Border{radius:2.into(),..Default::default()},..Style::default()});
  let indicator:Slot<'static,Message>=match style{"dashed"=>column![swatch(4,3),swatch(4,3),swatch(4,3)].spacing(2).into(),"line"=>swatch(4,20).into(),_=>swatch(9,9).into()};
  items.push(row![indicator,text::muted(name).size(12.0).width(Fill),text::small(value).medium()].spacing(6).width(Fill).into());
 }
 container(column(items).spacing(4).width(Fill)).width(132).padding(9).style(|t|Style{background:Some(t.neutral.base.into()),border:Border{width:1.0,color:t.neutral.border,radius:8.into()},..Default::default()}).into()
}
fn anatomy()->Slot<'static,Message>{
 let first=sample(Some("Page Views"),&[("Desktop","186",BLUE),("Mobile","80",LIGHT_BLUE)],"dot");
 let second=sample(None,&[("Chrome","1,286",BLUE),("Firefox","1,000",LIGHT_BLUE)],"dashed");
 let third=sample(Some("Page Views"),&[("Desktop","12,486",BLUE)],"line");
 let fourth=sample(None,&[("Chrome","1,286",LIGHT_BLUE)],"dot");
 column![
  row![column![text::small("Label"),first].spacing(8),column![text::small("Name"),second].spacing(8)].spacing(24),
  row![third,column![fourth,text::small("Indicator")].spacing(8)].spacing(24),
 ].spacing(18).into()
}
```
