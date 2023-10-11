-- Add migration script here
CREATE OR REPLACE FUNCTION update_modified_at()
    RETURNS TRIGGER AS $$
BEGIN
    NEW.modified_at = NOW();
    RETURN NEW;
END;
$$ LANGUAGE plpgsql;



CREATE TRIGGER trigger_update_modified_at
    BEFORE UPDATE ON secured_notes
    FOR EACH ROW
EXECUTE FUNCTION update_modified_at();
