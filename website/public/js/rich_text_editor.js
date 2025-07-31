class RichTextEditor {
    constructor() {
        this.editor = document.getElementById('editor');
        this.preview = document.getElementById('preview');
        this.imageInput = document.getElementById('imageInput');
        
        this.showHeadingMenu = false;
        this.showFormatMenu = false;
        this.content = '';
        this.storageKey = 'rich_editor_content';
        
        this.headingOptions = [
            { label: 'Heading 1', tag: 'h1', fontSize: '24px' },
            { label: 'Heading 2', tag: 'h2', fontSize: '18px' },
            { label: 'Heading 3', tag: 'h3', fontSize: '18px' },
            { label: 'Body', tag: 'p', fontSize: '16px' },
            { label: 'H5', tag: 'h5', fontSize: '14px' },
            { label: 'H6', tag: 'h6', fontSize: '12px' }
        ];

        this.formatOptions = [
            { label: 'Normal', command: 'removeFormat' },
            { label: 'Bold', command: 'bold' },
            { label: 'Italic', command: 'italic' }
        ];
        
        this.init();
    }

    init() {
        this.loadFromStorage();
        this.setupEventListeners();
        this.updatePreview();
    }

    setupEventListeners() {
        // Heading dropdown
        const headingBtn = document.getElementById('headingBtn');
        const headingMenu = document.getElementById('headingMenu');
        
        headingBtn.addEventListener('click', (e) => {
            e.stopPropagation();
            this.toggleDropdown('heading');
        });

        headingMenu.addEventListener('click', (e) => {
            if (e.target.tagName === 'BUTTON') {
                const tag = e.target.dataset.tag;
                const size = e.target.dataset.size;
                this.applyHeading(tag, size);
            }
        });

        // Format dropdown
        const formatBtn = document.getElementById('formatBtn');
        const formatMenu = document.getElementById('formatMenu');
        
        formatBtn.addEventListener('click', (e) => {
            e.stopPropagation();
            this.toggleDropdown('format');
        });

        formatMenu.addEventListener('click', (e) => {
            if (e.target.tagName === 'BUTTON') {
                const command = e.target.dataset.command;
                this.applyFormat(command);
            }
        });

        // Image upload
        document.getElementById('imageBtn').addEventListener('click', () => {
            this.triggerImageUpload();
        });

        this.imageInput.addEventListener('change', (e) => {
            this.handleImageUpload(e);
        });

        // List button
        document.getElementById('listBtn').addEventListener('click', () => {
            this.insertBulletList();
        });

        // Checkbox button
        document.getElementById('checkboxBtn').addEventListener('click', () => {
            this.insertCheckbox();
        });

        // Editor content changes
        this.editor.addEventListener('input', (e) => {
            this.content = e.target.innerHTML;
            this.saveToStorage();
            this.updatePreview();
        });

        // Update content on mouse up to handle formatting changes
        this.editor.addEventListener('mouseup', () => {
            setTimeout(() => {
                const selection = window.getSelection();
                if (selection.rangeCount > 0) {
                    // This helps maintain focus and selection
                }
            }, 10);
        });

        // Close dropdowns when clicking outside
        document.addEventListener('click', () => {
            this.closeAllDropdowns();
        });
    }

    toggleDropdown(type) {
        const headingMenu = document.getElementById('headingMenu');
        const formatMenu = document.getElementById('formatMenu');
        const headingContainer = headingMenu.parentElement;
        const formatContainer = formatMenu.parentElement;

        if (type === 'heading') {
            this.showHeadingMenu = !this.showHeadingMenu;
            headingMenu.classList.toggle('show', this.showHeadingMenu);
            headingContainer.classList.toggle('active', this.showHeadingMenu);
            
            // Close format menu
            this.showFormatMenu = false;
            formatMenu.classList.remove('show');
            formatContainer.classList.remove('active');
        } else if (type === 'format') {
            this.showFormatMenu = !this.showFormatMenu;
            formatMenu.classList.toggle('show', this.showFormatMenu);
            formatContainer.classList.toggle('active', this.showFormatMenu);
            
            // Close heading menu
            this.showHeadingMenu = false;
            headingMenu.classList.remove('show');
            headingContainer.classList.remove('active');
        }
    }

    closeAllDropdowns() {
        this.showHeadingMenu = false;
        this.showFormatMenu = false;
        
        const headingMenu = document.getElementById('headingMenu');
        const formatMenu = document.getElementById('formatMenu');
        
        if (headingMenu && formatMenu) {
            headingMenu.classList.remove('show');
            formatMenu.classList.remove('show');
            headingMenu.parentElement.classList.remove('active');
            formatMenu.parentElement.classList.remove('active');
        }
    }

    execCommand(command, value = null) {
        // Ensure editor is focused
        if (this.editor) {
            this.editor.focus();
        }
        
        // For better browser compatibility
        try {
            document.execCommand(command, false, value);
        } catch (e) {
            console.warn('Command not supported:', command);
        }
        
        this.saveToStorage();
        this.updatePreview();
    }

    applyHeading(tag, fontSize) {
        this.execCommand('formatBlock', `<${tag}>`);
        
        // Apply font size to the current selection/block
        const selection = window.getSelection();
        if (selection.rangeCount > 0) {
            const range = selection.getRangeAt(0);
            let element = range.commonAncestorContainer;
            
            // Find the block element
            while (element && element.nodeType !== 1) {
                element = element.parentNode;
            }
            
            if (element && element.tagName && element.tagName.toLowerCase() === tag) {
                element.style.fontSize = fontSize;
                if (tag !== 'p') {
                    element.style.fontWeight = 'bold';
                }
            }
        }
        
        this.closeAllDropdowns();
        this.saveToStorage();
        this.updatePreview();
    }

    applyFormat(command) {
        // Ensure editor is focused first
        if (this.editor) {
            this.editor.focus();
        }
        
        if (command === 'removeFormat') {
            // Remove all formatting but preserve text
            this.execCommand('removeFormat');
            // Reset to paragraph with default styling
            setTimeout(() => {
                this.execCommand('formatBlock', '<p>');
            }, 10);
        } else {
            // Apply bold or italic formatting
            this.execCommand(command);
        }
        
        this.closeAllDropdowns();
        this.saveToStorage();
        this.updatePreview();
    }

    insertBulletList() {
        this.execCommand('insertUnorderedList');
        this.saveToStorage();
    }

    insertCheckbox() {
        const checkbox = `<div contenteditable="false" style="display: inline-block; margin-right: 8px;">
            <input type="checkbox" style="margin-right: 4px;" />
        </div>`;
        this.execCommand('insertHTML', checkbox + '<span>Checkbox item</span><br>');
        this.saveToStorage();
    }

    triggerImageUpload() {
        // Ensure editor has focus before opening file dialog
        if (this.editor) {
            this.editor.focus();
        }
        if (this.imageInput) {
            this.imageInput.click();
        }
    }

    handleImageUpload(event) {
        const file = event.target.files[0];
        if (file && (file.type === 'image/png' || file.type === 'image/jpeg')) {
            const reader = new FileReader();
            reader.onload = (e) => {
                // Ensure editor has focus
                if (this.editor) {
                    this.editor.focus();
                }
                
                // Create image element with proper dimensions
                const img = document.createElement('img');
                img.onload = () => {
                    let width = img.width;
                    let height = img.height;
                    
                    // Set maximum dimensions
                    const maxWidth = 400;
                    const maxHeight = 300;
                    
                    // Calculate aspect ratio
                    const aspectRatio = width / height;
                    
                    // Resize based on aspect ratio while maintaining proportions
                    if (width > maxWidth) {
                        width = maxWidth;
                        height = width / aspectRatio;
                    }
                    
                    if (height > maxHeight) {
                        height = maxHeight;
                        width = height * aspectRatio;
                    }
                    
                    // Insert the image with calculated dimensions
                    const imageHTML = `<div><img src="${e.target.result}" style="width: ${Math.round(width)}px; height: ${Math.round(height)}px; max-width: 400px; max-height: 300px; object-fit: contain; margin: 10px 0; border-radius: 4px; box-shadow: 0 2px 8px rgba(0,0,0,0.1); display: block;" /></div><p><br></p>`;
                    
                    // Use different methods for better compatibility
                    if (document.queryCommandSupported('insertHTML')) {
                        document.execCommand('insertHTML', false, imageHTML);
                    } else {
                        // Fallback method
                        const selection = window.getSelection();
                        if (selection.rangeCount > 0) {
                            const range = selection.getRangeAt(0);
                            const div = document.createElement('div');
                            div.innerHTML = imageHTML;
                            range.insertNode(div.firstChild);
                            range.collapse(false);
                            selection.removeAllRanges();
                            selection.addRange(range);
                        }
                    }
                    
                    this.saveToStorage();
                    this.updatePreview();
                };
                img.src = e.target.result;
            };
            reader.readAsDataURL(file);
        }
        event.target.value = '';
    }

    updatePreview() {
        const content = this.editor ? this.editor.innerHTML : '';
        if (this.preview) {
            this.preview.innerHTML = content || '<p>Start typing your content here...</p>';
        }
    }

    saveToStorage() {
        if (this.editor) {
            const content = this.editor.innerHTML;
            try {
                localStorage.setItem(this.storageKey, content);
            } catch (e) {
                console.warn('Failed to save to localStorage:', e);
            }
        }
    }

    loadFromStorage() {
        try {
            const savedContent = localStorage.getItem(this.storageKey);
            if (savedContent && this.editor) {
                this.editor.innerHTML = savedContent;
                this.content = savedContent;
            }
        } catch (e) {
            console.warn('Failed to load from localStorage:', e);
        }
    }
}

// Initialize the editor when the DOM is loaded
document.addEventListener('DOMContentLoaded', () => {
    new RichTextEditor();
});