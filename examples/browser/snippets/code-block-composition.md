## Composition

```text
code_block![..]
├── code_block::header![..]
│   ├── code_block::title(..)
│   └── code_block::actions![..]
│       └── code_block::copy_button(..)
├── code_block::actions![..] (floating, when there is no header)
├── code_block::content(..)
│   └── code_block::line(..)
└── code_block::footer(..)
```

`code_block::content` takes a string or a list of `code_block::line`s. A string passed straight to `code_block!` is the content. Use lines when a row holds your own widgets.
