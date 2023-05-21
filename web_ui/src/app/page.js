
import styles from './page.module.css'
import ReactMarkdown from 'react-markdown'
import result_json from '../../../result.json'

const markdown = `
\`\`\`js
${JSON.stringify(result_json, null, 2)}
\`\`\`
`;

export default function Home() {
  return (
    <main className={styles.main}>
      {/* <pre>
        <code>
          {JSON.stringify(result_json, null, 2)}
        </code>
      </pre> */}
      <ReactMarkdown children={markdown} />
    </main>
  )
}
