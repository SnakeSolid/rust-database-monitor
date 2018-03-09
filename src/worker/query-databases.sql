SELECT
    d.datname,
    d.datcollate,
    r.rolname
FROM pg_database AS d
    INNER JOIN pg_roles AS r ON ( r.oid = d.datdba )
WHERE
    rolcreaterole = FALSE AND
    rolcanlogin = TRUE
