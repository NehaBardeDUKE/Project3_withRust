# Data Pre-processing microservice

A microservice that takes a the object uploaded to S3 and then pre-processes the data as required by the machine learning model using aws lambda.
For the purpose of this process, we simply tokenize the given sentence.


Project Outline:
![image](https://user-images.githubusercontent.com/110474064/227012425-dcf19fee-93b7-4627-aa82-6dd5d32fb0b8.png)

Here I first create a Lambda function called "bigdata_try2" using a docker image that i had pushed from my codespaces to the ECR. Then I create a S3 bucket and selected this lambda function as the destination for the event when a file lands on the S3 bucket.

S3 config:
![image](https://user-images.githubusercontent.com/110474064/227012839-5511533b-b920-4c04-b79b-1ef9ae6178d3.png)

![image](https://user-images.githubusercontent.com/110474064/227012953-7d111bfa-0ee0-4062-9f4c-0c5099f87608.png)

Lambda Function and success screenshot:
![image](https://user-images.githubusercontent.com/110474064/227013131-34fc3591-bda0-4b56-a0d4-dc1a26d17a68.png)

![image](https://user-images.githubusercontent.com/110474064/227013835-26870b1b-6747-4cb0-a5fe-485034e20b8e.png)

ECR repo:

![image](https://user-images.githubusercontent.com/110474064/227013677-c2c65742-8916-4a29-8a2d-1659ed70cda3.png)
