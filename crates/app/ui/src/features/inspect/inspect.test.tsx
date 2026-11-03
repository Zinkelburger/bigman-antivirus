import { screen, within } from "@testing-library/react";
import { describe, expect, it } from "vitest";
import { renderWithDaemon } from "@/test-utils";
import { InspectPage } from "./InspectPage";

describe("Inspect", () => {
  it("inspects an installer, highlights matched lines and previews a rehearsal", async () => {
    const { user } = renderWithDaemon(<InspectPage />);
    await user.type(await screen.findByLabelText("File, folder or HTTPS URL"), "/home/sam/Downloads/install-tool.sh");
    await user.click(screen.getByRole("button", { name: "Inspect" }));
    const result = await screen.findByRole("region", { name: "Result" });
    expect(within(result).getByText("Findings to review")).toBeInTheDocument();
    const script = within(result).getByRole("region", { name: "The script" });
    expect(within(script).getByText("shell.execute-download")).toBeInTheDocument();
    expect(within(script).getByText("shell.privilege")).toBeInTheDocument();
    await user.click(within(result).getByRole("button", { name: "Rehearse in the offline sandbox…" }));
    const dialog = await screen.findByRole("dialog", { name: "Rehearse the installer" });
    expect(await within(dialog).findByText(/does not install anything/)).toBeInTheDocument();
    expect(within(dialog).getByRole("button", { name: "Run in the sandbox" })).toBeDisabled();
  });

  it("reopens a past inspection with its full evidence, not the history summary", async () => {
    const { user } = renderWithDaemon(<InspectPage />);
    await user.click(await screen.findByRole("row", { name: /install-tool\.sh/ }));
    const panel = await screen.findByRole("dialog", { name: "Installer inspection" });
    expect(await within(panel).findByRole("region", { name: "The script" })).toBeInTheDocument();
    expect(within(panel).getByRole("button", { name: "Export report…" })).toBeInTheDocument();
  });

  it("routes a folder to the project inspector", async () => {
    const { user } = renderWithDaemon(<InspectPage />);
    await user.type(await screen.findByLabelText("File, folder or HTTPS URL"), "/home/sam/Projects/example-app");
    await user.click(screen.getByRole("button", { name: "Inspect" }));
    const result = await screen.findByRole("region", { name: "Result" });
    expect(within(result).getByRole("heading", { name: /^Project: / })).toBeInTheDocument();
    expect(within(result).getByRole("button", { name: "Disable automatic install scripts…" })).toBeInTheDocument();
  });

  it("compares a lockfile with an older copy", async () => {
    const { user } = renderWithDaemon(<InspectPage />);
    await user.click(await screen.findByText(/More inspections/));
    await user.type(screen.getByLabelText("Lockfile"), "/home/sam/app/package-lock.json");
    await user.type(screen.getByLabelText("Older copy of it"), "/home/sam/app-main/package-lock.json");
    await user.click(screen.getByRole("button", { name: "Compare lockfiles" }));
    const result = await screen.findByRole("region", { name: "Result" });
    expect(within(result).getByText("lockfile.source-changed")).toBeInTheDocument();
  });

  it("sends an HTTPS tarball to the installer inspector, as the daemon does", async () => {
    const { user } = renderWithDaemon(<InspectPage />);
    await user.type(await screen.findByLabelText("File, folder or HTTPS URL"), "https://example.org/tool.tgz");
    await user.click(screen.getByRole("button", { name: "Inspect" }));
    const result = await screen.findByRole("region", { name: "Result" });
    expect(within(result).getByRole("heading", { name: /^Installer/ })).toBeInTheDocument();
  });

  it("routes archives and launchers to their own checks and lists them afterwards", async () => {
    const { user } = renderWithDaemon(<InspectPage />);
    const field = await screen.findByLabelText("File, folder or HTTPS URL");
    await user.type(field, "/home/sam/Downloads/Meeting_Notice.pdf.desktop");
    await user.click(screen.getByRole("button", { name: "Inspect" }));
    let result = await screen.findByRole("region", { name: "Result" });
    expect(within(result).getByRole("heading", { name: /^Launcher: / })).toBeInTheDocument();
    expect(within(result).getByText("desktop.masquerade")).toBeInTheDocument();
    await user.click(screen.getByRole("button", { name: "Inspect something else" }));
    await user.type(await screen.findByLabelText("File, folder or HTTPS URL"), "/home/sam/Downloads/photos.zip");
    await user.click(screen.getByRole("button", { name: "Inspect" }));
    result = await screen.findByRole("region", { name: "Result" });
    expect(within(result).getByRole("heading", { name: /^Archive: / })).toBeInTheDocument();
    expect(within(result).getByText("archive.path-traversal")).toBeInTheDocument();
    expect((await screen.findAllByRole("row", { name: /^.*Launcher.*Meeting_Notice\.pdf\.desktop/ })).length).toBeGreaterThan(0);
  });
});
