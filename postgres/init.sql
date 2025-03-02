-- Create Users table
CREATE TABLE IF NOT EXISTS users (
    id UUID PRIMARY KEY,
    username VARCHAR(50) UNIQUE NOT NULL,
    email VARCHAR(100) UNIQUE NOT NULL,
    password VARCHAR(255) NOT NULL,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
    is_active BOOLEAN DEFAULT TRUE
);

-- Create Events table
CREATE TABLE IF NOT EXISTS events (
    id UUID PRIMARY KEY,
    name VARCHAR(255) NOT NULL,
    date VARCHAR(255) NOT NULL,
    venue VARCHAR(255) NOT NULL
);

-- Create Tickets table
CREATE TABLE IF NOT EXISTS tickets (
    id UUID PRIMARY KEY,
    event_id UUID NOT NULL,
    seat_number INT NOT NULL,
    price INT NOT NULL,
    status INT NOT NULL,
    FOREIGN KEY (event_id) REFERENCES events(id) ON DELETE CASCADE
);

-- Create Reservations table
CREATE TABLE IF NOT EXISTS reservations (
    id UUID PRIMARY KEY,
    ticket_id UUID NOT NULL,
    customer_id UUID,
    reservation_date VARCHAR(255) NOT NULL,
    status INT NOT NULL,
    FOREIGN KEY (ticket_id) REFERENCES tickets(id) ON DELETE CASCADE,
    FOREIGN KEY (customer_id) REFERENCES users(id) ON DELETE CASCADE
);


-- Example Inserts for testing

INSERT INTO users (id,username, email, password) 
VALUES ('b298e81a-057a-44bf-859b-d57920d451ae','test_user', 'test@example.com', 'hashed_password_here');

-- Insert sample events
INSERT INTO events (id, name, date, venue) VALUES
('c9d7d951-f343-4c9d-82d0-d2ef3d68b350', 'Rock Concert', '2025-06-01', 'Stadium Arena'),
('66d79560-1eaf-42e5-89b7-0c62f4658f83', 'Tech Conference', '2025-07-15', 'Conference Hall');

-- Insert sample tickets
INSERT INTO tickets (id, event_id, seat_number, price, status) VALUES
('3ffbfe47-4e5b-45fa-9c1f-8f350be02bdb', 'c9d7d951-f343-4c9d-82d0-d2ef3d68b350', 1, 100, 0),
('6ed22c79-d2f5-4d3b-8fa7-12c1bb02062b', '66d79560-1eaf-42e5-89b7-0c62f4658f83', 2, 200, 1),
('7a1b0adf-4583-4a0a-b8ca-585902495a01', 'c9d7d951-f343-4c9d-82d0-d2ef3d68b350', 3, 150, 2);

-- Insert sample reservations
INSERT INTO reservations (id, ticket_id, customer_id, reservation_date, status) VALUES
('4b07a70c-c0b6-45cf-bb7f-1048a935d1b0', '3ffbfe47-4e5b-45fa-9c1f-8f350be02bdb', 'b298e81a-057a-44bf-859b-d57920d451ae', '2025-05-10', 0),
('3b6e19b4-1e3f-4ab1-8d9e-b527c9982e24', '6ed22c79-d2f5-4d3b-8fa7-12c1bb02062b', 'b298e81a-057a-44bf-859b-d57920d451ae', '2025-06-01', 1),
('cf19f679-cf32-4707-98de-582da1a567c3', '7a1b0adf-4583-4a0a-b8ca-585902495a01', 'b298e81a-057a-44bf-859b-d57920d451ae', '2025-03-10', 2);



