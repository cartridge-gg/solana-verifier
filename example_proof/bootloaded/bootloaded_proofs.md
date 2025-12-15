Currently, only proofs after simple bootloader are supported. They should be created with the following config:
```
{
  "tasks": [
    {
      "type": "CairoPiePath",
      "path": "your absolute path to the pie file, e.g., ~/factorial.zip",
      "use_poseidon": true
    }
  ],
  "single_page": true
}
```