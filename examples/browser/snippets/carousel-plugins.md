## Autoplay

Set `.autoplay(Duration::from_secs(2))` to advance every two seconds. Hovering pauses the timer. When the pointer leaves, a fresh interval starts. Set `.stop_on_hover(true)` to stop autoplay after the first hover. Dragging or focusing an interactive slide also stops the timer. Autoplay wraps after the last snap.
