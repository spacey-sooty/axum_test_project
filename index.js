import { html, render, useEffect, useState } from 'https://esm.sh/htm/preact/standalone'

function StaticSysinfo(props) {
    return html`
        <h2>Static Sysinfo</h2>
        <ul>
            <li>Operating System: ${props.static_info[0]} ${props.static_info[2]}</li>
            <li>Kernel Version: ${props.static_info[1]}</li>
            <li>Host Name: ${props.static_info[3]}</li>
        </ul>
    `;
}

function App() {
    const [static_info, setStaticinfo] = useState([]);
    useEffect(() => {
        fetch('/api/static_sysinfo')
          .then((res) => {
            return res.json();
          })
          .then((data) => {
            setStaticinfo(data);
          });
      }, []);

    return html`
        <h1>Sysinfo Display</h1>
        <${StaticSysinfo} static_info=${static_info}/>
    `;
}

render(html`<${App} />`, document.body);
