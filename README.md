# rust-rocket-api-real-estate-demo

API to demonstrate a simple REST API using Rocket + Rust

## How do I run the web service:

```
cargo run
```

## How to interact with the API:

Get all real estates (empty at start) :

```
curl --header "Content-Type: application/json" http://localhost:8000/realestates/ | jq
```

Get one specific real estate by ID :

```
curl --header "Content-Type: application/json" http://localhost:8000/realestates/1 | jq
```

Add one specific real estate :

```
curl --header "Content-Type: application/json" \
  --request POST \                                     
  --data '{ "description": "My first real estate" }' \
  http://localhost:8000/realestates/1 | jq
```

Update one specific real estate by ID :

```
curl --header "Content-Type: application/json" \
  --request POST \                                     
  --data '{ "description": "My first real estate rebranded" }' \
  http://localhost:8000/realestates/1 | jq
```
