FROM frolvlad/alpine-glibc:alpine-3.9_glibc-2.29

WORKDIR /app
COPY target/release/my_k8s_dash /app/my_k8s_dash

ENTRYPOINT [ "/app/my_k8s_dash" ]
