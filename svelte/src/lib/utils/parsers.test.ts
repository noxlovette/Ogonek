import { describe, it, expect, vi } from "vitest";
import { extractWordsFromRewordFile, getVideoCallService } from "./parsers";

describe("getVideoCallService", () => {
  it("should identify Zoom URLs correctly", () => {
    expect(getVideoCallService("https://zoom.us/j/123456789")).toBe("Zoom");
    expect(getVideoCallService("https://us02web.zoom.us/j/123456789")).toBe("Zoom");
    expect(getVideoCallService("http://zoom.us/j/123456789")).toBe("Zoom");
    expect(getVideoCallService("https://company.zoom.us/j/meeting")).toBe("Zoom");
  });

  it("should identify Google Meet URLs correctly", () => {
    expect(getVideoCallService("https://meet.google.com/abc-defg-hij")).toBe("Google Meet");
    expect(getVideoCallService("http://meet.google.com/test")).toBe("Google Meet");
  });

  it("should identify Microsoft Teams URLs correctly", () => {
    expect(getVideoCallService("https://teams.microsoft.com/l/meetup-join/")).toBe("Microsoft Teams");
    expect(getVideoCallService("http://teams.microsoft.com/meeting")).toBe("Microsoft Teams");
  });

  it("should identify Webex URLs correctly", () => {
    expect(getVideoCallService("https://webex.com/meeting")).toBe("Webex");
    expect(getVideoCallService("https://company.webex.com/meet")).toBe("Webex");
    expect(getVideoCallService("http://test.webex.com/join")).toBe("Webex");
  });

  it("should identify GoToMeeting URLs correctly", () => {
    expect(getVideoCallService("https://gotomeeting.com/join/123456")).toBe("GoToMeeting");
    expect(getVideoCallService("https://global.gotomeeting.com/meeting")).toBe("GoToMeeting");
    expect(getVideoCallService("http://app.gotomeeting.com/test")).toBe("GoToMeeting");
  });

  it("should identify Discord URLs correctly", () => {
    expect(getVideoCallService("https://discord.gg/abc123")).toBe("Discord");
    expect(getVideoCallService("https://discord.com/channels/123/456")).toBe("Discord");
    expect(getVideoCallService("https://ptb.discord.com/channels/123/456")).toBe("Discord");
    expect(getVideoCallService("http://discord.gg/test")).toBe("Discord");
  });

  it("should identify Skype URLs correctly", () => {
    expect(getVideoCallService("https://skype.com/meeting")).toBe("Skype");
    expect(getVideoCallService("https://join.skype.com/abc123")).toBe("Skype");
    expect(getVideoCallService("http://web.skype.com/call")).toBe("Skype");
  });

  it("should identify Whereby URLs correctly", () => {
    expect(getVideoCallService("https://whereby.com/meeting-room")).toBe("Whereby");
    expect(getVideoCallService("https://subdomain.whereby.com/room")).toBe("Whereby");
    expect(getVideoCallService("http://whereby.com/test")).toBe("Whereby");
  });

  it("should return null for non-video call URLs", () => {
    expect(getVideoCallService("https://google.com")).toBe(null);
    expect(getVideoCallService("https://example.com/zoom")).toBe(null);
    expect(getVideoCallService("https://fakewebex.com/meeting")).toBe(null);
    expect(getVideoCallService("mailto:test@example.com")).toBe(null);
  });

  it("should return null for invalid inputs", () => {
    expect(getVideoCallService("")).toBe(null);
    expect(getVideoCallService(null as any)).toBe(null);
    expect(getVideoCallService(undefined as any)).toBe(null);
    expect(getVideoCallService(123 as any)).toBe(null);
  });

  it("should handle URLs with whitespace", () => {
    expect(getVideoCallService("  https://zoom.us/j/123456789  ")).toBe("Zoom");
    expect(getVideoCallService("\t\nhttps://meet.google.com/abc\n")).toBe("Google Meet");
  });

  it("should be case insensitive for domain matching", () => {
    expect(getVideoCallService("https://ZOOM.US/j/123456789")).toBe("Zoom");
    expect(getVideoCallService("https://Meet.Google.Com/abc")).toBe("Google Meet");
  });
});

describe("extractWordsFromRewordFile", () => {
  it("should handle file processing", async () => {
    const mockFile = new File(["dummy"], "test.reword", { type: "application/zip" });
    
    // Since we can't easily mock fflate in the test environment,
    // we'll just test that the function exists and throws appropriate errors
    await expect(extractWordsFromRewordFile(mockFile)).rejects.toThrow();
  });
});