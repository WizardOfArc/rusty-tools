# HTML Encoder
The purpose of this program is to convert HTML or JavaScript text into HTML entities for the purpose of inserting into an HTML page for display instead of execution

My plan is to create a parser of sorts that reads text character by character and creates tokens and appends them to a vector of Tokens

Then have a second pass that goes over that Vector of tokens and outputs the HTML encoded version.

There should be two main classifications of Tokens

1. literals - text that is safe to put in HTML as is
1. to_encode - text or symbols that need to be converted to html entities (examples: <, >, &, ;, ", ', {, and })