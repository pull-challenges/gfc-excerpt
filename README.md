# Example Excerpt to Submit for Translation and Evaluation

## About this Repo
* This repo is provided as what an example submission may look like <b>with the exception of the `./translated_rust/` directory</b>. This directory is provided as an example to show what translated code might look like, but shouldn't be submitted.

## Preparation for Translation
1. Be sure there is a `CMakeLists.txt` file present that adds an executable binary with the name `driver`. A minimalist example is provided [here](./test_case/CMakeLists.txt).
2. The excerpt will need to be tarballed to be run through the translator. The excerpt files should be in the root of the `.tar.gz` file, NOT in a subdirectory.
   1. To accomplish this, use the `tar -czvf some_project.tar.gz -C ./some_project/ .` command (Note the period at the end!)

## Preparation for Evaluation
1. Write `*.json` test vectors to be submitted alongside your code. Example test vectors have been provided [here](./test_vectors/).
2. Here is a schema that test vectors can take the form of:
```json
{
  "argv": ["--flag", "value"],                         // optional; arguments to be passed in through the CLI; appended after the driver path
  "stdin": "input string\n",                           // optional; input provided during execution
  "env": {                                             // optional; defaults to empty
    "ENV_VAR": "value"
  },
  "rc": 0,                                             // optional; return code; default 0
  "stdout": { "pattern": "ok\n", "is_regex": false },  // exact by default, otherwise if is_regex is true, then match by pattern
  "stderr": { "pattern": "", "is_regex": false },      // same as above
  "has_ub": "overflow"                                 // optional; used for checking undefined behavior; if present, test is skipped
}
```