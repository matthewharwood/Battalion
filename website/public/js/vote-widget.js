class VoteWidget extends HTMLElement {
  constructor() {
    super();
    this.form = this.querySelector('#vote-form');
    this.buttons = this.form.querySelectorAll('button');
    this.locked = false;
  }

  connectedCallback() {
    this.form.addEventListener('submit', (e) => {
      e.preventDefault();
      if (this.locked) {
        this.animateButtons();
        return;
      }
      const formData = new FormData(this.form);
      fetch('/vote', {
        method: 'POST',
        body: formData
      }).then(res => {
        if (res.status === 201) {
          this.locked = true;
        }
        this.animateButtons();
      });
    });

    const ws = new WebSocket('ws://localhost:8000');
    ws.onmessage = (msg) => {
      const data = JSON.parse(msg.data);
      if (data.applicantId === this.form.elements['applicantId'].value) {
        this.updateRarity(data.rarity);
      }
    };
  }

  animateButtons() {
    this.buttons.forEach(btn => {
      btn.classList.add('clicked');
      setTimeout(() => btn.classList.remove('clicked'), 300);
    });
  }

  updateRarity(val) {
    this.querySelector('#rarity-meter').textContent = `${val}%`;
  }
}

customElements.define('vote-widget', VoteWidget);
