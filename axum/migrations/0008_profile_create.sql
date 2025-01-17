-- Add migration script here
CREATE FUNCTION create_profile_for_user()
RETURNS TRIGGER AS $$
BEGIN
    INSERT INTO profiles (user_id)
    VALUES (NEW.id);
    RETURN NEW;
END;
$$ LANGUAGE plpgsql;

CREATE TRIGGER create_profile_after_user_insert
    AFTER INSERT ON "user"
    FOR EACH ROW
    EXECUTE FUNCTION create_profile_for_user();