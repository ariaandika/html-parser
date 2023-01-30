# HTML Parser
personal project for getting started on writing parser.


## Parsing result
```rust
// the Element tree container
struct TreeElement {
	tag: String,
	childs: Vec<Element>
}

struct Element {
	tag: String,
	attr: Vec<(String,String)>,
	scope: u16,
	inner: String,
	childs: Vec<Element>
}
```

## Supported
- tag, can contain alphanumeric and `-`
- attribute and innertext
- nested element
- sibling element

## Not yet implemented
- self close tag