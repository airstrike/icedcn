## Features

- Seven visual variants, from a strong primary bubble to unframed ghost content
- Start and end alignment for sender and receiver bubbles
- Reactions that anchor to the bubble edge with configurable side and alignment
- Bubbles size to their content, up to 80% of the container width
- Interactive surfaces through `.interactive(true)` or `.on_press(message)` for link and button behavior
- Customizable paint through `.map_style(...)`, content padding/radius, and reaction padding

On an interactive bubble, the entire padded surface receives pointer input, keyboard focus, Enter and Space. Use the published message in your application's `update` to navigate or perform an action. Local toast, collapsible and popover examples keep their own interaction state.
