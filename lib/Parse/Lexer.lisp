(defpackage :shift-lexer
  (:use :cl)
  (:export
   #:token
   #:token-kind
   #:token-value
   #:token-line
   #:token-column
   #:lex
   #:lex-file))

(in-package :shift-lexer)

(defstruct token
  kind
  value
  line
  column)

(defparameter *shift-keywords*
  '(
    "if"
    "else"
    "guard"
    "switch"
    "case"
    "default"

    "while"
    "for"
    "in"
    "loop"
    "repeat"
    "break"
    "continue"

    "true"
    "false"
    "nil"

    "var"
    "let"
    "const"
    "static"
    "lazy"
    "defer"

    "int"
    "float"
    "double"
    "bool"
    "string"
    "char"
    "bytes"
    "void"
    "any"
    "never"

    "struct"
    "class"
    "enum"
    "protocol"
    "extension"
    "typealias"
    "associatedtype"

    "func"
    "init"
    "deinit"
    "return"

    "self"
    "super"

    "import"
    "export"

    "public"
    "private"
    "internal"
    "fileprivate"
    "package"

    "get"
    "set"
    "willSet"
    "didSet"
    "subscript"

    "async"
    "await"
    "actor"
    "isolated"
    "nonisolated"

    "throws"
    "throw"
    "try"
    "catch"
    "finally"

    "where"
    "is"
    "as"
    "some"

    "operator"
    "precedence"
    "associativity"

    "move"
    "copy"
    "borrow"
    "consume"
    "owned"

    "weak"
    "unowned"

    "unsafe"
    "safe"

    "mutating"
    "nonmutating"

    "final"
    "override"
    "required"
    "convenience"

    "open"
    "dynamic"

    "inout"

    "yield"

    "macro"

    "sizeof"
    "typeof"

    "match"
    ))


(defun keyword-p (text)
  (member text *shift-keywords* :test #'string=))


(defstruct lexer
  source
  position
  length
  line
  column)


(defun make-shift-lexer (source)
  (make-lexer
   :source source
   :position 0
   :length (length source)
   :line 1
   :column 1))


(defun at-end-p (lexer)
  (>= (lexer-position lexer)
      (lexer-length lexer)))


(defun current-char (lexer)
  (unless (at-end-p lexer)
    (char (lexer-source lexer)
          (lexer-position lexer))))


(defun peek-char-at (lexer)
  (let ((position (1+ (lexer-position lexer))))
    (when (< position (lexer-length lexer))
      (char (lexer-source lexer) position))))


(defun advance (lexer)
  (unless (at-end-p lexer)
    (let ((character (current-char lexer)))
      (incf (lexer-position lexer))

      (if (char= character #\Newline)
          (progn
            (incf (lexer-line lexer))
            (setf (lexer-column lexer) 1))
          (incf (lexer-column lexer)))

      character)))


(defun whitespace-p (character)
  (member character
          '(#\Space #\Tab #\Newline #\Return)
          :test #'char=))


(defun digit-p (character)
  (and character
       (digit-char-p character)))


(defun identifier-start-p (character)
  (and character
       (or (alpha-char-p character)
           (char= character #\_))))


(defun identifier-character-p (character)
  (and character
       (or (identifier-start-p character)
           (digit-p character))))


(defun skip-whitespace (lexer)
  (loop
    while (and (not (at-end-p lexer))
               (whitespace-p (current-char lexer)))
    do (advance lexer)))


(defun skip-comment (lexer)
  (when (char= (current-char lexer) #\;)
    (loop
      while (and (not (at-end-p lexer))
                 (not (char= (current-char lexer) #\Newline)))
      do (advance lexer))))


(defun skip-space-and-comments (lexer)
  (loop
    (skip-whitespace lexer)

    (if (and (not (at-end-p lexer))
             (char= (current-char lexer) #\;))
        (skip-comment lexer)
        (return))))


(defun read-identifier (lexer)
  (let ((line (lexer-line lexer))
        (column (lexer-column lexer))
        (text
          (with-output-to-string (stream)
            (loop
              while (identifier-character-p (current-char lexer))
              do
                 (write-char (advance lexer) stream)))))

    (make-token
     :kind (if (keyword-p text)
               :keyword
               :identifier)
     :value text
     :line line
     :column column)))


(defun read-number (lexer)
  (let ((line (lexer-line lexer))
        (column (lexer-column lexer))
        (decimal nil)
        (text
          (with-output-to-string (stream)

            (loop
              while (digit-p (current-char lexer))
              do
                 (write-char (advance lexer) stream))

            (when (and (char= (or (current-char lexer) #\Space) #\.)
                       (digit-p (peek-char-at lexer)))
              (setf decimal t)
              (write-char (advance lexer) stream)

              (loop
                while (digit-p (current-char lexer))
                do
                   (write-char (advance lexer) stream))))))

    (make-token
     :kind (if decimal
               :number
               :integer)
     :value text
     :line line
     :column column)))


(defun read-string (lexer)
  (let ((line (lexer-line lexer))
        (column (lexer-column lexer)))

    (advance lexer)

    (let ((text
            (with-output-to-string (stream)
              (loop
                while (and (not (at-end-p lexer))
                           (not (char= (current-char lexer) #\")))
                do
                   (if (char= (current-char lexer) #\\)
                       (progn
                         (advance lexer)
                         (unless (at-end-p lexer)
                           (write-char (advance lexer) stream)))
                       (write-char (advance lexer) stream))))))

      (when (not (at-end-p lexer))
        (advance lexer))

      (make-token
       :kind :string
       :value text
       :line line
       :column column))))


(defun read-at-token (lexer)
  (let ((line (lexer-line lexer))
        (column (lexer-column lexer))
        (text
          (with-output-to-string (stream)
            (write-char (advance lexer) stream)

            (loop
              while (identifier-character-p (current-char lexer))
              do
                 (write-char (advance lexer) stream)))))

    (make-token
     :kind :directive
     :value text
     :line line
     :column column)))


(defun read-bang-token (lexer)
  (let ((line (lexer-line lexer))
        (column (lexer-column lexer))
        (text
          (with-output-to-string (stream)
            (write-char (advance lexer) stream)

            (loop
              while (identifier-character-p (current-char lexer))
              do
                 (write-char (advance lexer) stream)))))

    (if (keyword-p text)
        (make-token
         :kind :keyword
         :value text
         :line line
         :column column)

        (make-token
         :kind :operator
         :value text
         :line line
         :column column))))


(defun two-character-operator-p (first second)
  (member (list first second)
          '((#\= #\=)
            (#\! #\=)
            (#\< #\=)
            (#\> #\=)
            (#\- #\>)
            (#\+ #\+)
            (#\- #\-)
            (#\& #\&)
            (#\| #\|)
            (#\? #\?))
          :test #'equal))


(defun read-operator (lexer)
  (let ((line (lexer-line lexer))
        (column (lexer-column lexer))
        (first (current-char lexer))
        (second (peek-char-at lexer)))

    (if (two-character-operator-p first second)
        (progn
          (advance lexer)
          (advance lexer)

          (make-token
           :kind :operator
           :value (coerce (list first second) 'string)
           :line line
           :column column))

        (progn
          (advance lexer)

          (make-token
           :kind :operator
           :value (string first)
           :line line
           :column column)))))


(defun punctuation-p (character)
  (member character
          '(#\( #\)
            #\{ #\}
            #\[ #\]
            #\, #\: #\.
            #\?)
          :test #'char=))


(defun punctuation-kind (character)
  (case character
    (#\( :left-paren)
    (#\) :right-paren)
    (#\{ :left-brace)
    (#\} :right-brace)
    (#\[ :left-bracket)
    (#\] :right-bracket)
    (#\, :comma)
    (#\: :colon)
    (#\. :dot)
    (#\? :question)))


(defun read-punctuation (lexer)
  (let ((line (lexer-line lexer))
        (column (lexer-column lexer))
        (character (advance lexer)))

    (make-token
     :kind (punctuation-kind character)
     :value (string character)
     :line line
     :column column)))


(defun operator-character-p (character)
  (member character
          '(#\+
            #\-
            #\*
            #\/
            #\%
            #\=
            #\<
            #\>
            #\!
            #\&
            #\|)
          :test #'char=))


(defun next-token (lexer)
  (skip-space-and-comments lexer)

  (when (at-end-p lexer)
    (return-from next-token
      (make-token
       :kind :eof
       :value nil
       :line (lexer-line lexer)
       :column (lexer-column lexer))))

  (let ((character (current-char lexer)))

    (cond

      ((char= character #\")
       (read-string lexer))

      ((digit-p character)
       (read-number lexer))

      ((char= character #\@)
       (read-at-token lexer))

      ((char= character #\!)
       (read-bang-token lexer))

      ((identifier-start-p character)
       (read-identifier lexer))

      ((punctuation-p character)
       (read-punctuation lexer))

      ((operator-character-p character)
       (read-operator lexer))

      (t
       (advance lexer)

       (make-token
        :kind :unknown
        :value (string character)
        :line (lexer-line lexer)
        :column (lexer-column lexer))))))


(defun lex (source)
  (let ((lexer (make-shift-lexer source))
        (tokens '()))

    (loop
      for token = (next-token lexer)
      do (push token tokens)
      until (eq (token-kind token) :eof)

      finally
         (return (nreverse tokens)))))


(defun lex-file (pathname)
  (with-open-file (stream pathname
                          :direction :input
                          :external-format :utf-8)
    (lex
     (let ((text (make-string (file-length stream))))
       (read-sequence text stream)
       text))))