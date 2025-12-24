aws dynamodb list-tables --endpoint-url http://dynamodb-local:8000

aws dynamodb scan \
  --table-name users \
  --endpoint-url http://localhost:8000
