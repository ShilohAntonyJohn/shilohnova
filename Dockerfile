FROM fedora:42

RUN dnf install -y clang llvm binutils && dnf clean all

WORKDIR /app

COPY shilohnova .
COPY site ./site/

RUN mkdir /app/data

EXPOSE 3000

CMD ["./shilohnova"]
