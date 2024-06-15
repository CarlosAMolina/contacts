FROM python:3.12-slim

RUN mkdir -p /contacts

WORKDIR /contacts

COPY requirements.txt /contacts/

RUN pip install -r requirements.txt

# Copy the code after install dependencies because the code
# changes more often, as Docker only runs the changed step,
# this is more efficient.
COPY main.py /contacts/
COPY src /contacts/

EXPOSE 5000

CMD ["python", "main.py"]
