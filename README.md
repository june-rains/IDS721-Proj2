# IDS721-Proj2
[![Rust CI/CD Pipeline](https://github.com/june-rains/IDS721-Proj2/actions/workflows/rust.yml/badge.svg)](https://github.com/june-rains/IDS721-Proj2/actions/workflows/rust.yml)

***This is my study for Rust + Tokio Concurrency Library + Docker + MiniKube***

## Overview
This repo used **Rust** and **Tokio** to build a backend chat application, containerize this application with **Docker** then deploy with **MiniKube**


## Run
### Local 
  * Create a codespaces from my GitHub repo  
  * Run command `cargo run`  
  * Run command `telnet localhost 8080` in several other terminals
### MiniKube
  * Create a codespaces from my GitHub repo
  * Push container to DockerHub (Optional): I have already pushed my docker image to DockerHub, you can check by click this [link](https://hub.docker.com/repository/docker/junerains/chatapp/general)
  * Start MiniKube: `minikube start`
  * Create a deployment: `kubectl create deployment chatapp --image=registry.hub.docker.com/junerains/chatapp`
  * View deployment: `kubectl get deployments`
  * Create service and expose it (In my project, I choose **NodePort** type): `kubectl expose deployment chatapp --type=NodePort --port=8080`
  * Put forward port: `kubectl port-forward service/chatapp 8080:8080`
  * Run command: `telnet localhost 8080` in several other terminals
  * Clean up:
    ```bash
    kubectl delete service hello-fastapi
    kubectl delete deployment hello-fastapi
    minikube stop
    ````
 ## Results
 ### Local
 
 ### MiniKube
 
 ## Demo Video
 
 



