## Composition

```text
code_block(content)
├── .header(code_block::header(..))
│   ├── code_block::title(..)
│   └── .actions(code_block::actions![..])
│       └── code_block::copy_button(..)
├── .actions(..) (floating, when there is no header)
├── code_block::content(..)
│   └── code_block::line(..)
└── .footer(code_block::footer(..))
```

`code_block` takes a string, a list of `code_block::line`s or `code_block::content(..)`. Use lines when a row holds your own widgets.
