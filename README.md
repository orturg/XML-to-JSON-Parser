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

## Usage Description

Download parser, open it in code editor and open the console.
There are available console commands:
- Parse
- Instruction
- Credits

### Parse
To use this parser you will need file with XML language content in .txt or .xml format. Make sure your XML language content is valid. 
Put this file in the directory.
To start parser open terminal and write command cargo run -- parse your_file_name.xml. 
As a result program will print parsed XML into JSON.

```
cargo run -- parse your_file_name.xml
```

**Console input**

<img width="714" height="117" alt="Снимок экрана 2025-11-10 в 13 37 03" src="https://github.com/user-attachments/assets/5ff4bd75-948c-44fb-9fd6-bebe0fba741a" />

**xml.xml file content**

<img width="614" height="217" alt="Снимок экрана 2025-11-10 в 13 37 54" src="https://github.com/user-attachments/assets/c27fd680-bb83-4f71-b432-5d190ed7df3e" />

**Result**

<img width="552" height="134" alt="Снимок экрана 2025-11-10 в 13 39 07" src="https://github.com/user-attachments/assets/677a031b-10e9-4757-843f-31debed2d8f3" />

### Instruction
To use this command you will need to open the terminal and type cargo run -- instruction. Then instruction will appear in your console.

```
cargo run -- instruction
```
**Console input**

<img width="791" height="134" alt="Снимок экрана 2025-11-10 в 13 42 30" src="https://github.com/user-attachments/assets/1aa19f0b-cda0-4036-b642-417b4568ee88" />

**Result**

<img width="1091" height="125" alt="Снимок экрана 2025-11-10 в 13 43 06" src="https://github.com/user-attachments/assets/267af23e-aad1-452c-be3b-7496d3ef130a" />

### Credits
To use this command you will need to open the terminal and type cargo run -- credits. Then instruction will appear in your console.

```
cargo run -- credits
```
**Console input**

<img width="581" height="51" alt="Снимок экрана 2025-11-10 в 13 45 13" src="https://github.com/user-attachments/assets/2c0ffffd-a652-450c-9609-69e28b94f2a0" />

**Result**

<img width="627" height="69" alt="Снимок экрана 2025-11-10 в 13 45 27" src="https://github.com/user-attachments/assets/7113089c-7495-4f9a-acc5-14a9ae758e60" />

## Technical Description 
The parser analyzes XML input text, recognizes structural elements such as tags, attributes, and text nodes, and then transform these elements into a structured JSON representation. Firstly, XML input will be recognized by grammar rules using pest. Then the parsed input will be organized into a structure that consistes nested elements and relationships between XML elements. Finally, this structure will be converted into a JSON object.
