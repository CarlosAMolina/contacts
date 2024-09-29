FROM python:3.12-slim

RUN useradd --create-home --shell /bin/bash app_user

WORKDIR /home/app_user

COPY requirements.txt /home/app_user/

RUN pip install -r /home/app_user/requirements.txt

# Copy the code after install dependencies because the code
# changes more often, as Docker only runs the changed step,
# this is more efficient.
ADD src /home/app_user/

CMD ["bash"]
