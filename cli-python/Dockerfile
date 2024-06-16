FROM python:3.12-slim

RUN useradd --create-home --shell /bin/bash app_user

WORKDIR /home/app_user

COPY main.py /home/app_user/

CMD ["bash"]
