import { describe, expect, it } from "vitest";
import { isSuperUser } from "./constants";

describe("isSuperUser", () => {
  it("should return true for moderator role", () => {
    expect(isSuperUser("moderator")).toBe(true);
  });

  it("should return true for admin role", () => {
    expect(isSuperUser("admin")).toBe(true);
  });

  it("should return true for god role", () => {
    expect(isSuperUser("god")).toBe(true);
  });

  it("should return false for user role", () => {
    expect(isSuperUser("user")).toBe(false);
  });

  it("should return false for guest role", () => {
    expect(isSuperUser("guest")).toBe(false);
  });

  it("should return false for empty string", () => {
    expect(isSuperUser("")).toBe(false);
  });

  it("should return false for undefined role", () => {
    expect(isSuperUser(undefined as any)).toBe(false);
  });

  it("should return false for null role", () => {
    expect(isSuperUser(null as any)).toBe(false);
  });

  it("should be case sensitive", () => {
    expect(isSuperUser("ADMIN")).toBe(false);
    expect(isSuperUser("Admin")).toBe(false);
    expect(isSuperUser("MODERATOR")).toBe(false);
    expect(isSuperUser("GOD")).toBe(false);
  });

  it("should return false for partial matches", () => {
    expect(isSuperUser("mod")).toBe(false);
    expect(isSuperUser("administrator")).toBe(false);
    expect(isSuperUser("moderators")).toBe(false);
  });
});
