migrate:
	psql -U postgres -h localhost -W < migration/create_database.sql
	psql -U postgres -h localhost -W simulator < migration/layout.sql
	# mysql -u root -psecretpw --protocol tcp simulator < migration/layout.sql