# Privacy Policy

**Last updated:** April 2026

## Data Controller

Fabrizio Salmi — [GitHub](https://github.com/fabriziosalmi)

## What Data Is Processed

### Client-Side Processing (No Server Transmission)

The NIS2 Compliance Engine chatbot processes the following data **entirely in your browser**. No data is sent to any server operated by the project:

| Data | Purpose | Storage | Legal Basis (GDPR) |
|------|---------|---------|-------------------|
| Your questions (free text) | Matched against local Q&A dataset via BM25 search | In-memory only, cleared on page reload | Legitimate interest (Art. 6(1)(f)) |
| Numeric parameters (employees, revenue) | Real-time applicability assessment | In-memory only, cleared on page reload | Legitimate interest (Art. 6(1)(f)) |
| Browser language (`navigator.language`) | UI language auto-detection (IT/EN) | In-memory only | Legitimate interest (Art. 6(1)(f)) |
| Session state (visited categories) | Coverage tracking, follow-up suggestions | In-memory only, cleared on page reload | Legitimate interest (Art. 6(1)(f)) |

### Hosting Provider (GitHub Pages)

This site is hosted on [GitHub Pages](https://docs.github.com/en/pages/getting-started-with-github-pages/about-github-pages). GitHub may collect:

- IP addresses
- Browser user-agent strings
- Access timestamps

This data is processed by GitHub Inc. under their [Privacy Statement](https://docs.github.com/en/site-policy/privacy-policies/github-general-privacy-statement). GitHub acts as a data processor for hosting purposes.

## What Data Is NOT Collected

- **No cookies** are set by this application
- **No analytics** or tracking scripts are loaded
- **No personal data** is transmitted to any server operated by the project
- **No accounts** or registration are required
- **No data** is persisted after you close or reload the page

## REST API

If you deploy and use the REST API (`nis2-api` crate), company profile data (name, sector, employees, revenue) is processed **on your own server**. The API does not log or persist request data by default. Deployers are responsible for their own GDPR compliance.

## Your Rights (GDPR Art. 15-22)

Since all chatbot processing happens client-side with no server-side storage, there is no personal data held by the project to access, rectify, erase, or port. For data processed by GitHub Pages, exercise your rights directly with [GitHub](https://support.github.com/contact/privacy).

## Changes

This policy may be updated. Changes will be reflected in the "Last updated" date above.

## Contact

For privacy-related questions: open an issue on [GitHub](https://github.com/fabriziosalmi/nis2-model) or contact the data controller directly.
