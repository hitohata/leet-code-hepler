# Leet Code Helper

This is helper of [the leet code extension by 力扣 LeetCode](https://github.com/LeetCode-OpenSource/vscode-leetcode).
That creates code template on the top of project. 
This project once detects it and moves it to appropriate folder.

## Requirement

You need a language and extension mapping file.
The data format is .json, the file name must be `longuage.json`, and must be placed root of the project.
The data format is as following:

```json
[
  {
    "languageName": "rust",
    "languageExtension": "rs" 
  }
]
```