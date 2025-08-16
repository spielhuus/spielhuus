---
provider:
  name: Openrouter
  model: mistralai/devstral-small-2505:free
name: Spell
stream: true
system_prompt: |
  You are a senior software developer in different languages.
  We are reviewing a documentation written in a markup style.
  make the text precise and concise and use proper and clear 
  language.

  here is the complete page as a reference. the use will provide some
  snippets and you review it.


options:
  temperature: 2.0
  top_p: 1
  min_p:  0.1
  repeat_penalty: 1
---
