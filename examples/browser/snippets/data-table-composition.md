## Introduction

Each data table is built around its own records. This payments table uses `table::grid` for its shared column tracks, and a local component owns the filter, sort order, column visibility, selection and page.

## Basic Table

Define the columns once and build the header and body from the same visible list. The actions and selection columns stay visible. Give the email track the remaining width and let the other data columns size to their contents.

## Cell Formatting

Format the amount as a right-aligned dollar value with two decimal places. Render email in lowercase and status with an initial capital.

## Row Actions

Attach a dropdown to the final cell. Copy payment ID writes that row's original identifier to the clipboard.

## Pagination

Slice the filtered and sorted indices into pages of ten. Previous and Next disable at the ends. The five payments in this example fit on one page.

## Sorting

Activate the Email heading to alternate ascending and descending order. Sorting retains the selection by record ID.

## Filtering

The email input matches a case-insensitive substring. Filtering starts at the first page and an empty result spans the table.

## Visibility

The Columns menu can hide Status, Email and Amount independently. Header and body share the resulting track list.

## Row Selection

The header checkbox selects the current page. Its indeterminate state reflects a partial selection. The footer counts selected records in the current filter.
