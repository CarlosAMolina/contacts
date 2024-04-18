from sqlalchemy import Column
from sqlalchemy import ForeignKey
from sqlalchemy import func
from sqlalchemy import Integer
from sqlalchemy import String
from sqlalchemy.orm import relationship
from sqlalchemy.ext.declarative import declarative_base
from sqlalchemy.ext.hybrid import hybrid_property

from src.utils.unicode import ACCENT_TO_NO_ACCENT_MAP


Base = declarative_base()


class AddressModel(Base):
    __tablename__ = "addresses"

    id = Column(Integer, primary_key=True)
    id_user = Column(Integer, ForeignKey("users.id"), nullable=False)
    address = Column(String, nullable=False)
    user = relationship("UserModel", back_populates="addresses")

    @hybrid_property
    def address_unicode(self) -> Column[str]:
        result = self.address
        for old, new in ACCENT_TO_NO_ACCENT_MAP.items():
            result = func.REPLACE(result, old, new)
        return result


class EmailModel(Base):
    __tablename__ = "emails"

    id = Column(Integer, primary_key=True)
    id_user = Column(Integer, ForeignKey("users.id"), nullable=False)
    email = Column(String, nullable=False)
    user = relationship("UserModel", back_populates="emails")


class UserModel(Base):
    """
    relationship, back_populates: allow query other class values.
    """

    __tablename__ = "users"

    id = Column(Integer, primary_key=True)
    name = Column(String, nullable=False)
    surname = Column(String)
    addresses = relationship("AddressModel", back_populates="user")
    emails = relationship("EmailModel", back_populates="user")
