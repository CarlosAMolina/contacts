FROM python:3.12-slim

RUN mkdir /contacts

WORKDIR /contacts

COPY main.py /contacts/

# ENTRYPOINT ["tail", "-f", "/dev/null"]
CMD ["python", "main.py"]
