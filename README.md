# XML_to_JSON_Parser
Parser that converts simple XML language text to a string in JSON format written on Rust.

## Parser Overview
This parser takes string written in valid XML format (it can include tags, attributestags) and returns string written in JSON format.

## Example 

### Input
```
<parser>
    <title id = "1">XML_to_JSON</title>
    <author>Artur Nozhenko</author>
</parser>
```
### Output
```
{
	"parser": {
		"title": {
			"_id": "1",
			"_text": "XML_to_JSON"
		},
		"author": "Artur Nozhenko"
	}
}
```

## Technical Description 
The parser analyzes XML input text, recognizes structural elements such as tags, attributes, and text nodes, and then transform these elements into a structured JSON representation. Firstly, XML input will be recognized by grammar rules using pest. Then the parsed input will be organized into a structure that consistes nested elements and relationships between XML elements. Finally, this structure will be converted into a JSON object.
