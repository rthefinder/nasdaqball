import Dashboard from '@/components/Dashboard'
import Header from '@/components/Header'
import Footer from '@/components/Footer'

export default function Home() {
  return (
    <main className="min-h-screen flex flex-col">
      <Header />
      <Dashboard />
      <Footer />
    </main>
  )
}
