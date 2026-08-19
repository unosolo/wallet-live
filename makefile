# ==========================================
# CONFIGURATION
# ==========================================

PROJECT=wallet-live

BASE_PATH=D:\containers\\$(PROJECT)
DB_CONFIG_PATH=$(BASE_PATH)\postgres_config
CADDY_CONFIG_PATH=$(BASE_PATH)\caddy_config

# volumes
DATA_ROOT_PATH=D:\containers
VOL_POSTGRES=global_postgres_data
VOL_CADDY=$(PROJECT)_caddy_data
#VOL_POSTGRES=$(DATA_ROOT_PATH)\$(PROJECT)\data\postgres
#VOL_CADDY=$(DATA_ROOT_PATH)\$(PROJECT)\data\caddy

# Replace <directory-path> with the path where you created folders earlier
DATA_FOLDER=$(VOL_CADDY)

DOMAIN_NAME=my.local
SUBDOMAIN=beelink-windows
GENERIC_TIMEZONE=America/Sao_Paulo

# The email address to use for the SSL certificate creation
SSL_EMAIL=example@example.com

POSTGRES_USER=admin
POSTGRES_PASSWORD=zzzzzzzzzzzzz
POSTGRES_DB=my_wallet_live_db

POSTGRES_NON_ROOT_USER=julio
POSTGRES_NON_ROOT_PASSWORD=julio

DOMAIN=$(SUBDOMAIN).$(DOMAIN_NAME)
DB_NAME=$(POSTGRES_DB)
DB_USER=$(POSTGRES_USER)
DB_PASS=$(POSTGRES_PASSWORD)
NON_ROOT_USER=$(POSTGRES_NON_ROOT_USER)
NON_ROOT_PASS=$(POSTGRES_NON_ROOT_PASSWORD)

# containers name
POSTGRES_CONTAINER_HOST=global_postgres
CADDY_CONTAINER_HOST=$(PROJECT)_caddy

# shared network between containers
NETWORK=$(PROJECT)_network


.PHONY: all setup run start stop restart clean logs

# Default: setup and create containers
all: setup run

# 1. Infrastructure setup
setup:
	@echo "Creating network and directories..."
	-wslc network create $(NETWORK)
	@echo "Creating volumes/directories..."
	-mkdir $(VOL_POSTGRES)
	-mkdir $(VOL_CADDY)

# 2. CREATE and run (used the first time or for updates)
run:
	@echo "Creating and starting containers for the first time..."
	-$(MAKE) run-postgres
	-$(MAKE) run-caddy

run-postgres:
	wslc run -d \
		--name $(POSTGRES_CONTAINER_HOST) \
		-p 5432:5432 \
		--env-file $(BASE_PATH)/.env \
		-e POSTGRES_DB=$(DB_NAME) \
		-v $(VOL_POSTGRES):/var/lib/postgresql/data \
		-v $(DB_CONFIG_PATH)\init-data.sh:/docker-entrypoint-initdb.d/init-data.sh \
		postgres:16
	timeout /t 5 || sleep 5
	$(MAKE) run-db-check

run-db-check:
	wslc exec $(POSTGRES_CONTAINER_HOST) pg_isready -U $(DB_USER) -d $(DB_NAME)

run-caddy:
	wslc run -d \
		--name $(CADDY_CONTAINER_HOST) \
		-p 80:80 \
		-p 443:443 \
		-v $(CADDY_CONFIG_PATH):/config \
		-v $(CADDY_CONFIG_PATH)\Caddyfile:/etc/caddy/Caddyfile \
		caddy:latest

# 3. START (Wake up existing containers)
start:
	@echo "Waking up existing containers..."
	-wslc start $(POSTGRES_CONTAINER_HOST)
	-wslc start $(CADDY_CONTAINER_HOST)

# 4. STOP (Shut down containers)
stop:
	@echo "Stopping containers..."
	-wslc stop $(POSTGRES_CONTAINER_HOST)
	-wslc stop $(CADDY_CONTAINER_HOST)

restart: stop start
	timeout /t 5 || sleep 5
	$(MAKE) run-db-check

# 5. RESTART (Full cycle: stop, remove, and run again)
# Use this if you changed a variable in the Makefile and want it to take effect
rerun: stop
	@echo "Removing old containers to apply changes..."
	-wslc rm -f $(POSTGRES_CONTAINER_HOST)
	-wslc rm -f $(CADDY_CONTAINER_HOST)
	$(MAKE) run

rebuild: stop clean
	$(MAKE) run

# 6. Log viewing
logs:
	wslc logs -f wallet-live_app

# 7. Total Wipe
clean:
	@echo "Removing old containers to apply changes..."
	wslc rm -f $(POSTGRES_CONTAINER_HOST) $(CADDY_CONTAINER_HOST)
	@echo "Removing old volumes to apply changes..."
	wslc volume remove $(VOL_POSTGRES)
	@echo "Removing old network to apply changes..."
	wslc network rm $(NETWORK)

psql:
	wslc exec -it $(POSTGRES_CONTAINER_HOST) psql -U admin -d $(DB_NAME)
