curl https://s3.eu-central-1.amazonaws.com/dynamodb-local-frankfurt/dynamodb_local_latest.zip --output dynamodb.zip
unzip ./dynamodb.zip -d ./dynamodb/ && rm -rf ./dynamodb.zip
java -Djava.library.path=./dynamodb/DynamoDBLocal_lib -jar ./dynamodb/DynamoDBLocal.jar -sharedDb