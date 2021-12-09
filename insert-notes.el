(defun goto-org-content-start ()
  "In current buffer, take point to where the org content
  starts."
  (beginning-of-buffer)
  (while (string-match "[:#]" (buffer-substring (point) (1+ (point))))
    (next-line)
    (beginning-of-line)))

(defun get-note-as-md (&optional title)
  "get note with TITLE from org-roam as markdown"
  (let* ((node-file (org-roam-node-file
                     ;; (org-roam-node-from-title-or-alias title)
                     (org-roam-node-read title)))
         (throwaway-str "Throwaway header to force org-md export to export h2s")
         (org-content (with-temp-buffer
                        (org-mode)
                        (insert-file-contents node-file)
                        (org-with-point-at 1
                          (org-map-entries 'org-do-demote))
                        (goto-org-content-start)
                        (insert (concat "\n* " throwaway-str "\n"))
                        (buffer-string)))
         (md-content (org-export-string-as org-content 'md t '(:with-toc nil))))
    (with-temp-buffer
      (insert md-content)
      (substring-no-properties (buffer-string)
                               (progn (goto-char (point-min))
                                      (search-forward throwaway-str)
                                      (next-line)
                                      (beginning-of-line)
                                      (point))
                               (1- (point-max))))))

(defun rust-docify (str)
  "Convert STR into rust documentation comment by prefixing each line with ///"
  (let ((result nil)
        (comment-prefix "/// ")
        (lines (split-string str "\n")))
    (dolist (line lines)
      (push (concat "/// " line) result))
    (string-join (reverse result ) "\n")))

(defun insert-note-as-rs-doc ()
  "Insert org-roam note as rust documentation string at point."
  (interactive)
  (insert (rust-docify (get-note-as-md))))
