-- Sort  (cost=283.10..283.25 rows=61 width=2259) (actual time=1.737..1.745 rows=15 loops=1)
explain analyze
    SELECT 
      users.id, 
      users.name, 
      users.surname, 
      nicknames.nickname, 
      phones.phone, 
      phones.description AS phone_description, 
      categories.category, 
      addresses.address, 
      emails.email, 
      urls.url, 
      facebook.url AS facebook_url, 
      twitter.handle AS twitter_handle, 
      instagram.handle AS instagram_handle, 
      notes.note 
    FROM 
      (
        (
          (
            (
              (
                (
                  (
                    (
                      (
                        (
                          (
                            contacts.users 
                            LEFT JOIN contacts.addresses ON (
                              (users.id = addresses.id_user)
                            )
                          ) 
                          LEFT JOIN contacts.users_categories ON (
                            (
                              users.id = users_categories.id_user
                            )
                          )
                        ) 
                        LEFT JOIN contacts.categories ON (
                          (
                            users_categories.id_category = categories.id
                          )
                        )
                      ) 
                      LEFT JOIN contacts.emails ON (
                        (users.id = emails.id_user)
                      )
                    ) 
                    LEFT JOIN contacts.facebook ON (
                      (users.id = facebook.id_user)
                    )
                  ) 
                  LEFT JOIN contacts.instagram ON (
                    (users.id = instagram.id_user)
                  )
                ) 
                LEFT JOIN contacts.nicknames ON (
                  (users.id = nicknames.id_user)
                )
              ) 
              LEFT JOIN contacts.notes ON (
                (users.id = notes.id_user)
              )
            ) 
            LEFT JOIN contacts.phones ON (
              (users.id = phones.id_user)
            )
          ) 
          LEFT JOIN contacts.twitter ON (
            (users.id = twitter.id_user)
          )
        ) 
        LEFT JOIN contacts.urls ON (
          (users.id = urls.id_user)
        )
      )
where 
  users.id in (
    SELECT 
      id_user 
    from 
      contacts.addresses 
    WHERE 
      TRANSLATE(
        LOWER(address), 
        'áéíóú', 
        'aeiou'
      ) LIKE '%arlos%' 
    union 
    SELECT 
      id_user 
    from 
      contacts.emails 
    WHERE 
      TRANSLATE(
        LOWER(email), 
        'áéíóú', 
        'aeiou'
      ) LIKE '%arlos%' 
    union 
    SELECT 
      id_user 
    from 
      contacts.facebook 
    WHERE 
      TRANSLATE(
        LOWER(url), 
        'áéíóú', 
        'aeiou'
      ) LIKE '%arlos%' 
    union 
    SELECT 
      id_user 
    from 
      contacts.instagram 
    WHERE 
      TRANSLATE(
        LOWER(handle), 
        'áéíóú', 
        'aeiou'
      ) LIKE '%arlos%' 
    union 
    SELECT 
      id_user 
    from 
      contacts.nicknames 
    WHERE 
      TRANSLATE(
        LOWER(nickname), 
        'áéíóú', 
        'aeiou'
      ) LIKE '%arlos%' 
    union 
    SELECT 
      id_user 
    from 
      contacts.notes 
    WHERE 
      TRANSLATE(
        LOWER(note), 
        'áéíóú', 
        'aeiou'
      ) LIKE '%arlos%' 
    union 
    SELECT 
      id_user 
    from 
      contacts.phones 
    WHERE 
      CAST(phone AS TEXT) LIKE '%arlos%' 
      or TRANSLATE(
        LOWER(description), 
        'áéíóú', 
        'aeiou'
      ) LIKE '%arlos%' 
    union 
    SELECT 
      id_user 
    from 
      contacts.twitter 
    WHERE 
      TRANSLATE(
        LOWER(handle), 
        'áéíóú', 
        'aeiou'
      ) LIKE '%arlos%' 
    union 
    SELECT 
      id_user 
    from 
      contacts.urls 
    WHERE 
      TRANSLATE(
        LOWER(url), 
        'áéíóú', 
        'aeiou'
      ) LIKE '%arlos%' 
    union 
    SELECT 
      id 
    from 
      contacts.users 
    WHERE 
      TRANSLATE(
        LOWER(
          CONCAT_WS(' ', name, surname)
        ), 
        'áéíóú', 
        'aeiou'
      ) LIKE '%arlos%'
  ) 
ORDER BY 
  LOWER(
    CONCAT_WS(' ', name, surname)
  ) ASC;

-- analyze new query -- not use view, get id
-- HashAggregate  (cost=120.07..120.17 rows=10 width=4)
explain (
SELECT
  id_user
from
  contacts.addresses
WHERE
  TRANSLATE(
    LOWER(address),
    'áéíóú',
    'aeiou'
  ) LIKE '%arlos%'
union
SELECT
  id_user
from
  contacts.emails
WHERE
  TRANSLATE(
    LOWER(email),
    'áéíóú',
    'aeiou'
  ) LIKE '%arlos%'
union
SELECT
  id_user
from
  contacts.facebook
WHERE
  TRANSLATE(
    LOWER(url),
    'áéíóú',
    'aeiou'
  ) LIKE '%arlos%'
union
SELECT
  id_user
from
  contacts.instagram
WHERE
  TRANSLATE(
    LOWER(handle),
    'áéíóú',
    'aeiou'
  ) LIKE '%arlos%'
union
SELECT
  id_user
from
  contacts.nicknames
WHERE
  TRANSLATE(
    LOWER(nickname),
    'áéíóú',
    'aeiou'
  ) LIKE '%arlos%'
union
SELECT
  id_user
from
  contacts.notes
WHERE
  TRANSLATE(
    LOWER(note),
    'áéíóú',
    'aeiou'
  ) LIKE '%arlos%'
union
SELECT
  id_user
from
  contacts.phones
WHERE
  CAST(phone AS TEXT) LIKE '%arlos%'
  or TRANSLATE(
    LOWER(description),
    'áéíóú',
    'aeiou'
  ) LIKE '%arlos%'
union
SELECT
  id_user
from
  contacts.twitter
WHERE
  TRANSLATE(
    LOWER(handle),
    'áéíóú',
    'aeiou'
  ) LIKE '%arlos%'
union
SELECT
  id_user
from
  contacts.urls
WHERE
  TRANSLATE(
    LOWER(url),
    'áéíóú',
    'aeiou'
  ) LIKE '%arlos%'
union
SELECT
  id
from
  contacts.users
WHERE
  TRANSLATE(
    LOWER(
      CONCAT_WS(' ', name, surname)
    ),
    'áéíóú',
    'aeiou'
  ) LIKE '%arlos%'
 );



-- analyze view query
-- only id
-- Hash Left Join  (cost=182.95..247.68 rows=9 width=4)

explain
                 SELECT
                   id
                 from
                   contacts.all_data
                 WHERE
                   TRANSLATE(
                     LOWER(
                       CONCAT_WS(
                         ' ',
                         name,
                         surname,
                         nickname,
                         phone,
                         phone_description,
                         category,
                         address,
                         email,
                         url,
                         facebook_url,
                         twitter_handle,
                         instagram_handle,
                         note
                       )
                     ),
                     'áéíóú',
                     'aeiou'
                   ) LIKE '%arlos%'


-- using view all data
-- Sort  (cost=485.74..485.87 rows=55 width=2259) (actual time=3.670..3.682 rows=15 loops=1)

explain analyze
            SELECT
               *
             from
               contacts.all_data
             WHERE
               id IN (
                 SELECT
                   id
                 from
                   contacts.all_data
                 WHERE
                   TRANSLATE(
                     LOWER(
                       CONCAT_WS(
                         ' ',
                         name,
                         surname,
                         nickname,
                         phone,
                         phone_description,
                         category,
                         address,
                         email,
                         url,
                         facebook_url,
                         twitter_handle,
                         instagram_handle,
                         note
                       )
                     ),
                     'áéíóú',
                     'aeiou'
                   ) LIKE '%arlos%'
               )
             ORDER BY
               LOWER(
                 CONCAT_WS(' ', name, surname)
               ) ASC


-- not usin view, create it in the query
-- only id
-- Hash Left Join  (cost=182.95..247.68 rows=9 width=4)
explain (
                 SELECT
                   id
                 from
                   ( 
 SELECT users.id,
    users.name,
    users.surname,
    nicknames.nickname,
    phones.phone,
    phones.description AS phone_description,
    categories.category,
    addresses.address,
    emails.email,
    urls.url,
    facebook.url AS facebook_url,
    twitter.handle AS twitter_handle,
    instagram.handle AS instagram_handle,
    notes.note
   FROM (((((((((((contacts.users
     LEFT JOIN contacts.addresses ON ((users.id = addresses.id_user)))
     LEFT JOIN contacts.users_categories ON ((users.id = users_categories.id_user)))
     LEFT JOIN contacts.categories ON ((users_categories.id_category = categories.id)))
     LEFT JOIN contacts.emails ON ((users.id = emails.id_user)))
     LEFT JOIN contacts.facebook ON ((users.id = facebook.id_user)))
     LEFT JOIN contacts.instagram ON ((users.id = instagram.id_user)))
     LEFT JOIN contacts.nicknames ON ((users.id = nicknames.id_user)))
     LEFT JOIN contacts.notes ON ((users.id = notes.id_user)))
     LEFT JOIN contacts.phones ON ((users.id = phones.id_user)))
     LEFT JOIN contacts.twitter ON ((users.id = twitter.id_user)))
     LEFT JOIN contacts.urls ON ((users.id = urls.id_user)))
                   ) all_data
                 WHERE
                   TRANSLATE(
                     LOWER(
                       CONCAT_WS(
                         ' ',
                         name,
                         surname,
                         nickname,
                         phone,
                         phone_description,
                         category,
                         address,
                         email,
                         url,
                         facebook_url,
                         twitter_handle,
                         instagram_handle,
                         note
                       )
                     ),
                     'áéíóú',
                     'aeiou'
                   ) LIKE '%arlos%'
);
