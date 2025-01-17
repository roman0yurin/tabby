import { Content, Editor, EditorEvents } from '@tiptap/react'
import { ListFileItem } from 'tabby-chat-panel/index'

/**
 * PromptProps defines the props for the PromptForm component.
 */
export interface PromptProps {
  /**
   * A callback function that handles form submission.
   * Returns a Promise for handling async operations.
   */
  onSubmit: (value: string) => Promise<void>
  /**
   * Indicates whether the form (or chat) is in a loading/submitting state.
   */
  isLoading: boolean
  onUpdate?: (p: EditorEvents['update']) => void
}

/**
 * PromptFormRef defines the methods exposed by PromptForm via forwardRef.
 */
export interface PromptFormRef {
  /**
   * Focuses the editor within PromptForm.
   */
  focus: () => void
  /**
   * Programmatically sets the editor's text content.
   */
  setInput: (value: Content) => void
  /**
   * Returns the current text content of the editor.
   */
  input: string
  editor: Editor | null
}

/**
 * Represents a file item in the workspace.
 */
export type FileItem = ListFileItem

/**
 * Represents a file source item for the mention suggestion list.
 */
export interface SourceItem {
  name: string
  filepath: string
  category: 'file'
  fileItem: FileItem
}

/**
 * Defines the attributes to be stored in a mention node.
 */
export interface MentionNodeAttrs {
  category: 'file'
  filepath: string
}

/**
 * Maintains the current state of the mention feature while typing.
 */
export interface MentionState {
  items: SourceItem[]
  command: ((props: MentionNodeAttrs) => void) | null
  query: string
  selectedIndex: number
}
