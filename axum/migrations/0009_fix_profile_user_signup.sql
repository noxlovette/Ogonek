-- Add migration script here
CREATE OR REPLACE FUNCTION create_profile_for_user() 
RETURNS TRIGGER AS $$
BEGIN
 INSERT INTO profile (user_id)
 VALUES (NEW.id);
 RETURN NEW;
END;
$$ LANGUAGE plpgsql;

-- coagulate with the previous migration!