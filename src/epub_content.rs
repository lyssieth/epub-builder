// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with
// this file, You can obtain one at https://mozilla.org/MPL/2.0/.

use crate::Element;

use std::io::Read;

/// Represents the possible reference type of an EPUB page.
///
/// Used by the guide section of EPUB 2.0 and the lankmarks navigation section
/// for EPUB 3.0.
///
/// For more information, see <http://www.idpf.org/epub/20/spec/OPF_2.0.1_draft.htm#Section2.3>
/// and <https://idpf.github.io/epub-vocabs/structure/>
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ReferenceType {
    /// The Book cover(s) (this refers to the cover PAGE, not the cover IMAGE)
    Cover,
    /// Page with title, author, publisher
    TitlePage,
    /// Table of contents
    Toc,
    /// Index
    Index,
    /// Glossary
    Glossary,
    /// Aknowledgements
    Acknowledgements,
    /// Bibliography
    Bibliography,
    /// No idea what this might be
    Colophon,
    /// Copyright page
    Copyright,
    /// Dedication
    Dedication,
    /// Epigraph
    Epigraph,
    /// Foreword
    Foreword,
    /// List of illustrations
    Loi,
    /// List of tables
    Lot,
    /// Notes
    Notes,
    /// Preface
    Preface,
    /// Beginning of the real content
    Text,
}

/// Represents a XHTML file that can be added to an EPUB document.
///
/// This struct is designed to be used with the `add_content` method
/// of the [`EpubBuilder`](struct.EpubBuilder.html).
///
/// # Example
///
/// ```
/// use epub_builder::{EpubContent, Element};
///
/// let page_content = "Some XHTML content";
///
/// // Creates a new EpubContent
/// let content = EpubContent::new("intro.xhtml", page_content.as_bytes())
/// // ... and sets a title so it is added to the TOC
///     .title("Introduction")
/// // ... and add some toc information on the document structure
///     .child(Element::new("intro.xhtml#1", "Section 1"))
///     .child(Element::new("intro.xhtml#2", "Section 2"));
/// ```
#[derive(Debug)]
pub struct EpubContent<R: Read> {
    /// The title and url, plus sublevels
    pub toc: Element,
    /// The content
    pub content: R,
    /// Properties. See [EpubProperties](enum.EpubProperties.html)
    pub reftype: Option<ReferenceType>,
}

impl<R: Read> EpubContent<R> {
    /// Creates a new `EpubContent`
    ///
    /// By default, this element is at level 1, and it has no title
    /// (meaning it won't be added to the [`Table of Contents`](struct.Toc.html).
    pub fn new<S: Into<String>>(href: S, content: R) -> Self {
        Self {
            content,
            toc: Element::new(href, ""),
            reftype: None,
        }
    }

    /// Set the title of this content. If no title is set,
    /// this part of the book will not be displayed in the table of content.
    #[must_use]
    pub fn title<S: Into<String>>(mut self, title: S) -> Self {
        self.toc.title = title.into();
        self
    }

    /// Set the level
    #[must_use]
    #[allow(clippy::missing_const_for_fn)]
    pub fn level(mut self, level: i32) -> Self {
        self.toc = self.toc.level(level);
        self
    }

    /// Adds a sublevel to the toc
    #[must_use]
    pub fn child(mut self, elem: Element) -> Self {
        self.toc = self.toc.child(elem);
        self
    }

    /// Sets reference type of this content
    ///
    /// If this is set, this will list this item as a reference in the guide section.
    ///
    /// See `<www.idpf.org/epub/20/spec/OPF_2.0.1_draft.htm#Section2.3>`
    ///
    /// # Example
    ///
    /// Reference an item as the title page:
    ///
    /// ```
    /// use epub_builder::{EpubContent, ReferenceType};
    /// let dummy = "Should be a XHTML file";
    /// let item = EpubContent::new("title.xhtml", dummy.as_bytes())
    ///      .title("Title")
    ///      .reftype(ReferenceType::TitlePage);
    /// ```
    #[must_use]
    pub const fn reftype(mut self, reftype: ReferenceType) -> Self {
        self.reftype = Some(reftype);
        self
    }
}
