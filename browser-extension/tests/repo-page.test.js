const { repoFromPage, summarize } = require('../repo-page');

test('a page on a code host maps to its repository', () => {
    expect(repoFromPage('https://github.com/owner/take-home')).toBe('https://github.com/owner/take-home');
    expect(repoFromPage('https://github.com/owner/take-home/tree/main/src?x=1#readme')).toBe('https://github.com/owner/take-home');
    expect(repoFromPage('https://www.github.com/owner/take-home.git')).toBe('https://github.com/owner/take-home');
    expect(repoFromPage('https://gitlab.com/group/sub/project/-/blob/main/README.md')).toBe('https://gitlab.com/group/sub/project');
    expect(repoFromPage('https://codeberg.org/owner/repo/src/branch/main')).toBe('https://codeberg.org/owner/repo');
    expect(repoFromPage('https://bitbucket.org/team/repo/src/main/')).toBe('https://bitbucket.org/team/repo');
});

test('other pages are not repositories', () => {
    for (const url of [
        'https://github.com/owner',
        'https://github.com/features/copilot',
        'https://github.com/settings/keys',
        'https://github.com/orgs/acme/repositories',
        'http://github.com/owner/repo',
        'https://example.org/owner/repo',
        'https://github.com/owner/-repo',
        'not a url'
    ]) expect(repoFromPage(url)).toBeNull();
});

test('a result lists findings by severity and says what to do after a High one', () => {
    const report = {
        findings: [
            { severity: 'info', summary: 'package.json script: dev' },
            { severity: 'review', summary: 'A web address is stored encoded' },
            { severity: 'high', summary: 'server/config/conf.js runs text it fetches or decodes as code' }
        ],
        coverage: { checked: [], notes: ['Read before cloning: git fetched the latest commit (abc123).'], skipped: [] }
    };
    const s = summarize({ status: 'FINDINGS', reason: 'Findings to review', report });
    expect(s.findings).toEqual([
        { severity: 'High', summary: 'server/config/conf.js runs text it fetches or decodes as code' },
        { severity: 'Review', summary: 'A web address is stored encoded' }
    ]);
    expect(s.read).toContain('abc123');
    expect(s.next).toContain('bigman try');
    expect(summarize({ status: 'NO_FINDINGS', reason: 'No findings from these checks', report: { ...report, findings: [] } }).next).toBeNull();
});

test('a missing desktop app is reported, not hidden', () => {
    const s = summarize({ status: 'INCOMPLETE', reason: 'Specified native messaging host not found.' });
    expect(s.status).toBe('INCOMPLETE: Specified native messaging host not found.');
    expect(s.findings).toEqual([]);
});
