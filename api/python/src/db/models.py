from sqlalchemy import Column
from sqlalchemy import ForeignKey
from sqlalchemy import Integer
from sqlalchemy import String
from sqlalchemy.orm import relationship
from sqlalchemy.ext.declarative import declarative_base


Base = declarative_base()


# TODO rename to UserModel
class User(Base):
    __tablename__ = "users"

    id = Column(Integer, primary_key=True)
    name = Column(String, nullable=False)
    surname = Column(String)
    emails = relationship("Email", back_populates="user")  # Allows query other class values.


# TODO rename to EmailModel
class Email(Base):
    __tablename__ = "emails"

    id = Column(Integer, primary_key=True)
    id_user = Column(Integer, ForeignKey("users.id"), nullable=False)
    email = Column(String, nullable=False)
    user = relationship("User", back_populates="emails")  # Allows query other class values.
