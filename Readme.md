### Run HTTP Server

- cargo run
- curl -s http://localhost:8080/users | jq .

<pre>
[
  {
    "id": 1,
    "name": "Alice"
  },
  {
    "id": 2,
    "name": "Bob"
  }
]
</pre>
