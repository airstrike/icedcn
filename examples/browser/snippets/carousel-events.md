## Events

With `.on_value_change(Message::SlideChanged)`, the carousel publishes the requested slide index when the user navigates or autoplay advances. To control the carousel, store the index in `update` and pass it back with `.value(index)`.
