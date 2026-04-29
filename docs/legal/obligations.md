# Obligations (Art. 20, 21, 23)

The engine maps 16 obligations from three articles of Directive (EU) 2022/2555. All are defined in `crates/rules/src/obligations.rs`.

## Art. 20 -- Governance (2 obligations)

| ID | Article | Description |
|----|---------|-------------|
| `nis2_art20_1` | Art. 20(1) | Management body approval of cybersecurity risk measures and oversight of implementation |
| `nis2_art20_2` | Art. 20(2) | Mandatory cybersecurity training for management body members |

## Art. 21(2) -- Cybersecurity measures (10 obligations)

| ID | Article | Description |
|----|---------|-------------|
| `nis2_art21_2_a` | Art. 21(2)(a) | Risk analysis and information system security policies |
| `nis2_art21_2_b` | Art. 21(2)(b) | Incident handling |
| `nis2_art21_2_c` | Art. 21(2)(c) | Business continuity and disaster recovery |
| `nis2_art21_2_d` | Art. 21(2)(d) | Supply chain security |
| `nis2_art21_2_e` | Art. 21(2)(e) | Security in system acquisition, development, and maintenance |
| `nis2_art21_2_f` | Art. 21(2)(f) | Policies to assess effectiveness of cybersecurity measures |
| `nis2_art21_2_g` | Art. 21(2)(g) | Basic cyber hygiene and cybersecurity training |
| `nis2_art21_2_h` | Art. 21(2)(h) | Cryptography and encryption policies |
| `nis2_art21_2_i` | Art. 21(2)(i) | Human resources security, access control, and asset management |
| `nis2_art21_2_j` | Art. 21(2)(j) | Multi-factor authentication, secured communications, emergency systems |

## Art. 23 -- Incident reporting (4 obligations)

| ID | Article | Description |
|----|---------|-------------|
| `nis2_art23_1` | Art. 23(1) | Notify CSIRT of significant incidents without undue delay |
| `nis2_art23_4_a` | Art. 23(4)(a) | Early warning within 24 hours |
| `nis2_art23_4_b` | Art. 23(4)(b) | Full incident notification within 72 hours |
| `nis2_art23_4_d` | Art. 23(4)(d) | Final report within 1 month |

All obligations carry Italian-language `legal_text` fields sourced from the directive. All start with `ObligationStatus::Pending`.
