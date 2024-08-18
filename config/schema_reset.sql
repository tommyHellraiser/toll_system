###############
# Drop tables
###############
DROP TABLE IF EXISTS transit_logs;
DROP TABLE IF EXISTS transit_fee_logs;
DROP TABLE IF EXISTS penalties;
DROP TABLE IF EXISTS violation_logs;
DROP TABLE IF EXISTS registered_vehicles;
DROP TABLE IF EXISTS discounts;
DROP TABLE IF EXISTS transit_rates;
DROP TABLE IF EXISTS clients_balances;
DROP TABLE IF EXISTS blacklist;
DROP TABLE IF EXISTS transaction_logs;
DROP TABLE IF EXISTS vehicle_types;
DROP TABLE IF EXISTS booths;
DROP TABLE IF EXISTS clients;


###############
# Schema reset
###############
CREATE TABLE IF NOT EXISTS clients (
    ID int PRIMARY KEY AUTO_INCREMENT,
    first_name varchar(40) NOT NULL,
    last_name varchar(40) NOT NULL,
    document varchar(20) UNIQUE NOT NULL,
    email varchar(60) UNIQUE NOT NULL,
    date_of_birth date NOT NULL,
    phone_number varchar(30) NULL DEFAULT NULL,
    address varchar(100) NULL DEFAULT NULL,
    is_active boolean DEFAULT true,
    created_at datetime DEFAULT NOW(),
    updated_at datetime NULL DEFAULT NULL ON UPDATE NOW(),
    INDEX clients_document (document),
    INDEX clients_email (email)
);

CREATE TABLE IF NOT EXISTS booths (
    ID int PRIMARY KEY AUTO_INCREMENT,
    city varchar(30) NOT NULL,
    route varchar(30) NOT NULL,
    max_lanes tinyint NOT NULL,
    is_active boolean DEFAULT true,
    created_at datetime DEFAULT NOW(),
    updated_at datetime NULL DEFAULT NULL ON UPDATE NOW(),
    deleted_at datetime NULL DEFAULT NULL
);

CREATE TABLE IF NOT EXISTS vehicle_types (
    ID int PRIMARY KEY AUTO_INCREMENT,
    vehicle_type enum('Bike', 'Car', 'Pickup', 'Van', 'Truck', 'Trailer', 'Service', 'Bus') NOT NULL,
    created_at datetime DEFAULT NOW(),
    updated_at datetime NULL DEFAULT NULL ON UPDATE NOW()
);

CREATE TABLE IF NOT EXISTS transaction_logs (
    ID int PRIMARY KEY AUTO_INCREMENT,
    clients_ID int,
    amount decimal(12,2) DEFAULT 0,
    payment_method enum('Prepaid', 'Cash', 'Credit', 'Debit', 'Other') NOT NULL,
    origin enum('BalanceRecharge', 'Transit', 'Penalty'),
    status enum('Confirmed', 'Failed', 'Unknown') NOT NULL DEFAULT 'Unknown',
    event_ID int NOT NULL COMMENT 'This field contains the ID of the origin table where it was originated from. It its origin is a transit, it will contain transit_logs.ID',
    transaction_time datetime NOT NULL DEFAULT NOW(),
    external_reference_id varchar(30) NULL DEFAULT NULL,
    error varchar(128) NULL DEFAULT NULL,
    created_at datetime NOT NULL DEFAULT NOW(),
    updated_at datetime NULL DEFAULT NULL ON UPDATE NOW(),
    CONSTRAINT transaction_logs_clients_ID FOREIGN KEY (clients_ID) REFERENCES clients (ID) ON UPDATE CASCADE ON DELETE CASCADE,
    INDEX transaction_logs_clients_ID (clients_ID),
    INDEX transaction_logs_event_ID (event_ID),
    INDEX transaction_logs_transaction_time (transaction_time)
);

CREATE TABLE IF NOT EXISTS blacklist (
    ID int PRIMARY KEY AUTO_INCREMENT,
    clients_ID int NULL DEFAULT NULL,
    license_plate varchar(10) NOT NULL,
    reason varchar(128) NOT NULL,
    restriction_expiry datetime DEFAULT NULL,
    created_at datetime NOT NULL DEFAULT NOW(),
    updated_at datetime NULL DEFAULT NULL ON UPDATE NOW(),
    CONSTRAINT blacklist_clients_ID FOREIGN KEY (clients_ID) REFERENCES clients (ID) ON UPDATE CASCADE ON DELETE CASCADE,
    INDEX blacklist_clients_ID (clients_ID)
);

CREATE TABLE IF NOT EXISTS clients_balances (
    ID int PRIMARY KEY AUTO_INCREMENT,
    clients_ID int NOT NULL,
    balance decimal(12,2) DEFAULT 0,
    created_at datetime NOT NULL DEFAULT NOW(),
    updated_at datetime NULL DEFAULT NULL ON UPDATE NOW(),
    CONSTRAINT clients_balances_clients_ID FOREIGN KEY (clients_ID) REFERENCES clients (ID) ON UPDATE CASCADE ON DELETE CASCADE,
    INDEX clients_balances_clients_ID (clients_ID)
);

CREATE TABLE IF NOT EXISTS transit_rates (
    ID int PRIMARY KEY AUTO_INCREMENT,
    vehicle_types_ID int NOT NULL,
    fee_amount decimal(12,2) NOT NULL,
    booths_ID int NULL DEFAULT NULL COMMENT 'If booth ID is NULL, then this rate is applied globally for all booths',
    valid_from datetime NOT NULL DEFAULT NOW(),
    valid_until datetime NULL DEFAULT NULL,
    created_at datetime DEFAULT NOW(),
    updated_at datetime DEFAULT NULL ON UPDATE NOW(),
    CONSTRAINT transit_rates_vehicle_types_ID FOREIGN KEY (vehicle_types_ID) REFERENCES vehicle_types (ID) ON UPDATE CASCADE ON DELETE CASCADE,
    CONSTRAINT transit_rates_booths_ID FOREIGN KEY  (booths_ID) REFERENCES booths (ID) ON UPDATE CASCADE ON DELETE CASCADE,
    INDEX transit_rates_vehicle_types_ID (vehicle_types_ID),
    INDEX transit_rates_booths_ID (booths_ID)
);

CREATE TABLE IF NOT EXISTS discounts (
    ID int PRIMARY KEY AUTO_INCREMENT,
    vehicle_types_ID int NULL DEFAULT NULL COMMENT 'If vehicle types ID is NULL, this discount is applied to all types of vehicles',
    valid_from datetime NOT NULL DEFAULT NOW(),
    valid_until datetime NULL DEFAULT NULL,
    discount_percentage decimal(12,2) NOT NULL DEFAULT 0 COMMENT '0 = 0% discount (No discount) and 1 = 100% discount (Client does not pay)',
    booths_ID int NULL DEFAULT NULL COMMENT 'If booth ID is NULL, then the discount is applied to all booths',
    description varchar(100) NULL DEFAULT NULL,
    created_at datetime DEFAULT NULL,
    updated_at datetime NULL DEFAULT NULL ON UPDATE NOW(),
    CONSTRAINT discounts_vehicle_types_ID FOREIGN KEY (vehicle_types_ID) REFERENCES vehicle_types (ID) ON UPDATE CASCADE ON DELETE CASCADE,
    CONSTRAINT discounts_booths_ID FOREIGN KEY (booths_ID) REFERENCES booths (ID) ON UPDATE CASCADE ON DELETE CASCADE,
    INDEX discounts_valid_from (valid_from),
    INDEX discounts_valid_until (valid_until)
);

CREATE TABLE IF NOT EXISTS registered_vehicles (
    ID int PRIMARY KEY AUTO_INCREMENT,
    clients_ID int NOT NULL,
    vehicle_types_ID int NOT NULL,
    license_plate varchar(10) UNIQUE NOT NULL,
    color varchar(20) NOT NULL DEFAULT '',
    brand varchar(20) NOT NULL DEFAULT '',
    model varchar(20) NOT NULL DEFAULT '',
    year varchar(20) NOT NULL DEFAULT '',
    active_until datetime NULL DEFAULT NULL,
    created_at datetime NOT NULL DEFAULT NOW(),
    updated_at datetime NULL DEFAULT NULL ON UPDATE NOW(),
    CONSTRAINT registered_vehicles_clients_ID FOREIGN KEY (clients_ID) REFERENCES clients (ID) ON UPDATE CASCADE ON DELETE CASCADE,
    CONSTRAINT registered_vehicles_vehicle_types_ID FOREIGN KEY (vehicle_types_ID) REFERENCES vehicle_types (ID) ON UPDATE CASCADE ON DELETE CASCADE,
    INDEX registered_vehicles_clients_ID (clients_ID)
);

CREATE TABLE IF NOT EXISTS violation_logs (
    ID int PRIMARY KEY AUTO_INCREMENT,
    clients_ID int,
    registered_vehicles_ID int,
    license_plate varchar(10) NOT NULL,
    description varchar (128) NOT NULL,
    created_at datetime NOT NULL DEFAULT NOW(),
    updated_at datetime NULL DEFAULT NULL ON UPDATE NOW(),
    CONSTRAINT violation_logs_clients_ID FOREIGN KEY (clients_ID) REFERENCES clients (ID) ON UPDATE CASCADE ON DELETE CASCADE,
    CONSTRAINT violation_logs_registered_vehicles_ID FOREIGN KEY (registered_vehicles_ID) REFERENCES registered_vehicles (ID) ON UPDATE CASCADE ON DELETE CASCADE,
    INDEX violation_logs_clients_ID (clients_ID),
    INDEX violation_logs_license_plate (license_plate)
);

CREATE TABLE IF NOT EXISTS penalties (
    ID int PRIMARY KEY AUTO_INCREMENT,
    violation_logs_ID int,
    clients_ID int,
    registered_vehicles_ID int,
    license_plate varchar(10) NOT NULL,
    penalty_amount decimal(12,2) DEFAULT 0,
    reason varchar(128) NOT NULL,
    penalty_paid boolean DEFAULT false,
    paid_at datetime NULL DEFAULT NULL,
    created_at datetime NOT NULL DEFAULT NOW(),
    updated_at datetime NULL DEFAULT NULL ON UPDATE NOW(),
    CONSTRAINT penalties_violation_logs_ID FOREIGN KEY (violation_logs_ID) REFERENCES violation_logs (ID) ON UPDATE CASCADE ON DELETE CASCADE,
    CONSTRAINT penalties_clients_ID FOREIGN KEY (clients_ID) REFERENCES clients (ID) ON UPDATE CASCADE ON DELETE CASCADE,
    CONSTRAINT penalties_registered_vehicles_ID FOREIGN KEY (registered_vehicles_ID) REFERENCES registered_vehicles (ID) ON UPDATE CASCADE ON DELETE CASCADE,
    INDEX penalties_clients_ID (clients_ID),
    INDEX penalties_registered_vehicles_ID (registered_vehicles_ID),
    INDEX penalties_license_plate (license_plate)
);

CREATE TABLE IF NOT EXISTS transit_fee_logs (
    ID int PRIMARY KEY AUTO_INCREMENT,
    original_amount decimal(12,2) NOT NULL DEFAULT 0,
    has_client_discount boolean NOT NULL DEFAULT false,
    transit_rates_ID int,
    discounts_ID int,
    final_amount decimal(12,2) DEFAULT 0,
    payment_method enum('Prepaid', 'Cash', 'Credit', 'Debit', 'Other'),
    created_at datetime NOT NULL DEFAULT NOW(),
    updated_at datetime NULL DEFAULT NULL ON UPDATE NOW(),
    CONSTRAINT transit_logs_transit_rates_ID FOREIGN KEY (transit_rates_ID) REFERENCES transit_rates (ID) ON UPDATE CASCADE ON DELETE CASCADE,
    CONSTRAINT transit_logs_discounts_ID FOREIGN KEY (discounts_ID) REFERENCES  discounts (ID) ON UPDATE CASCADE ON DELETE CASCADE
);

CREATE TABLE IF NOT EXISTS transit_logs (
    ID int PRIMARY KEY AUTO_INCREMENT,
    booths_ID int NOT NULL,
    clients_ID int,
    registered_vehicles_ID int,
    license_plate varchar(10),
    transit_time datetime NOT NULL DEFAULT NOW(),
    transit_fee_logs_ID int NOT NULL,
    violation_logs_ID int NULL DEFAULT NULL,
    created_at datetime NOT NULL DEFAULT NOW(),
    updated_at datetime NULL DEFAULT NULL ON UPDATE NOW(),
    CONSTRAINT transit_logs_booths_ID FOREIGN KEY (booths_ID) REFERENCES booths (ID) ON UPDATE CASCADE ON DELETE CASCADE,
    CONSTRAINT transit_logs_clients_ID FOREIGN KEY (clients_ID) REFERENCES clients (ID) ON UPDATE CASCADE ON DELETE CASCADE,
    CONSTRAINT transit_logs_registered_vehicles_ID FOREIGN KEY (registered_vehicles_ID) REFERENCES registered_vehicles (ID) ON UPDATE CASCADE ON DELETE CASCADE,
    CONSTRAINT transit_logs_transit_fee_logs_ID FOREIGN KEY  (transit_fee_logs_ID) REFERENCES transit_fee_logs (ID) ON UPDATE CASCADE ON DELETE CASCADE,
    CONSTRAINT transit_logs_violation_logs_ID FOREIGN KEY (violation_logs_ID) REFERENCES violation_logs (ID) ON UPDATE CASCADE ON DELETE CASCADE,
    INDEX transit_logs_transit_time (transit_time),
    INDEX transit_logs_license_plate (license_plate),
    INDEX transit_logs_clients_ID (clients_ID),
    INDEX transit_logs_registered_vehicles_ID (registered_vehicles_ID)
);


###############
# Data reset
###############
INSERT INTO vehicle_types (vehicle_type)
VALUES ('Bike'),
('Car'),
('Pickup'),
('Van'),
('Truck'),
('Trailer'),
('Service'),
('Bus');

INSERT INTO booths (city, route, max_lanes)
VALUES ('Buenos Aires', '7',4);

INSERT INTO discounts (vehicle_types_id, valid_from, valid_until, discount_percentage, booths_id)
VALUES (NULL, NOW(), NULL, 0.15, NULL);
