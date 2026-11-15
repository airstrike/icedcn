## The application model

An iced application holds its data in a state type. Its `view` composes widgets; events produce typed messages; `update` changes state and may return a `Task` for asynchronous work. A subscription connects continuing event sources to those messages.

Keep parsing, validation, and streamed-data updates in `update`. Build the interface from the resulting values in `view`. iced processes messages and requests redraws when needed.

icedcn controls can keep their own local state. Use `default_checked`, `default_value`, or `default_open` to seed an uncontrolled control. Supply `checked`, `value`, or `open` when the application owns that state, and handle the matching change message in `update`. The [Components guide](icedcn://guide/components#state-and-messages) gives a controlled example.

The library's children use `Slot`; an iced application boundary uses `iced::Element`. Both conversions are supported by the public builders.
