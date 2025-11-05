# XML-to-JSON-Parser
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

