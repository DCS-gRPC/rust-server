# Introduction

Thank you for contributing to DCS-gRPC. Before submitting your PR please read
through the following guide.

## Communication

This project uses Github for issue tracking and discussions however discussions
can also take place on Discord for a quicker feedback cycle. We recommend
joining https://discord.gg/9RqyTJt if you plan to make contributions. There is 
a `DCS-gRPC` category of channels and discussion takes place in the #developers
channel.

# Tenets

## KISS (Keep It Simple, Stupid) for clients

DCS-gRPC clients can be written in a wide variety of languages, each with their
own idiosyncracies. Therefore we will focus on keeping the gRPC interface as
simple as possible even if this means some extra verbosity or complexity inside
the DCS-gRPC server.

## Maintain consistency across the DCS-gRPC APIs

Try and maintain consistency in call patterns and field names in the DCS-gRPC
API. This may mean breaking from the  conventions in the underlying ED APIs
(See the next Tenet)

## Follow ED API conventions by default but do not be slaves to them.

We will follow the ED API conventions by default but this is a guide rather
than a rule. Renaming fields and APIs to make more sense is fine for example.

# Contributing Guidelines

## Document the gRPC interface

Add documentation to the gRPC .proto files using the proto-gen-doc format
detailed at https://github.com/pseudomuto/protoc-gen-doc#writing-documentation

## Follow git commit message best practices

Do not create 1 line commit messages for anything but the most trivial of commits.
Follow the recommendations in this template by default.

```plain
Capitalized, short (50 chars or less) summary

More detailed explanatory text, if necessary.  Wrap it to about 72
characters or so.  In some contexts, the first line is treated as the
subject of an email and the rest of the text as the body.  The blank
line separating the summary from the body is critical (unless you omit
the body entirely); tools like rebase can get confused if you run the
two together.

Try to pre-emptively answer any foreseeable "Why?" questions a reader
may have. There is no size limit on commit messages.

Write your commit message in the imperative: "Fix bug" and not "Fixed bug"
or "Fixes bug."  This convention matches up with commit messages generated
by commands like git merge and git revert.

Further paragraphs come after blank lines.

- Bullet points are okay, too

- Typically a hyphen or asterisk is used for the bullet, followed by a
  single space, with blank lines in between, but conventions vary here

- Use a hanging indent

If you use an issue tracker, add a reference(s) to them at the bottom,
like so:

Resolves: #123
```

## Squash commits and rebase before merging

When you are ready to merge then squash your commits so that they form a
series of logical Atomic commits. Depending on the size of the change this
might mean having one or a small series of commits.

## Use of linters

This project makes use of the following tools to lint the lua and .proto files

* [protolint](https://github.com/yoheimuta/protolint)
* [luacheck](https://github.com/mpeterv/luacheck)

It is not mandatory to run these yourself on your local machine however they
are run as part of the automated checks when you create a pull request so it
may save you time to run them yourself before-hand.